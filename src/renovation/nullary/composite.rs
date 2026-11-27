//! Compose independently reconstructed nonrecursive families and checked
//! ordinary declarations. Acceptance-only: omitted conversion rules
//! cannot justify rejection. No raw inductive enters the legacy
//! parser; only reconstructed constructor-specific iota is enabled.
//! No eta, projection, K, literal or axiom authority is installed.
use super::*;
use crate::convert::DeltaPolicy;
use crate::environment::{ConstantDecl,Environment};
use crate::id::NameId;
use crate::inductive::ClosedNonrecursiveDerivation;
use crate::level::LevelTerm;
use crate::syntax::Declaration;
use crate::typecheck::{TypeChecker,TypeValue};
use crate::value::EnvFrame;

enum Stage { Family(parametric::CertifiedFamily), Ordinary(usize) }

pub(super) fn check(rows:&[Value],meta:Value)->Option<super::super::Outcome>{
    need(rows.len()<=MAX_RECORDS)?;
    let mut tables=Tables::new();
    let mut prior=parametric::PriorFamilies::default();
    let mut stages=Vec::new();
    let mut names=HashSet::new();
    let mut ordinary_rows=Vec::new();
    let mut count=0usize;
    let mut families=0usize;
    for row in rows{
        if keys(row,&["inductive"]){
            let cert=prior.certify(&tables,row.get("inductive")?)?;
            need(cert.signatures.iter().all(|s|names.insert(s.name)))?;
            stages.push(Stage::Family(cert));
            families+=1;
            continue;
        }
        let (item,is_def)=if keys(row,&["def"]){(row.get("def")?,true)}
        else if keys(row,&["thm"]){(row.get("thm")?,false)}
        else {
            if keys(row,&["ie","letE"]){tables.add_ordinary_let(row)?;}
            else{tables.add(row)?;}
            ordinary_rows.push(row.clone());
            continue;
        };
        if is_def {
            need(keys(item,&["all","hints","levelParams","name","safety","type","value"]))?;
            need(item.get("safety")?.as_str()?=="safe")?;
            let h=item.get("hints")?;
            need(matches!(h.as_str(),Some("opaque"|"abbrev"))||(keys(h,&["regular"])&&num(h,"regular").is_some()))?;
        }else{need(keys(item,&["all","levelParams","name","type","value"]))?;}
        let name=num(item,"name")?;
        need(tables.valid_name(name)&&numbers(item,"all")?==[name])?;
        need(names.insert(NameId(u64::try_from(name).ok()?)))?;
        let ps=numbers(item,"levelParams")?;
        let mut unique=HashSet::new();
        need(ps.iter().all(|&n|tables.valid_name(n)&&unique.insert(n)))?;
        need(num(item,"type")?<tables.exprs.len()&&num(item,"value")?<tables.exprs.len())?;
        stages.push(Stage::Ordinary(count));
        count+=1;
        ordinary_rows.push(row.clone());
    }
    need(families>0&&count>0)?;
    // No version relabeling. Only ordinary and table rows enter
    // the shared native parser. Family authority is reconstructed.
    let export=crate::parser::parse_ordinary_native(meta,&ordinary_rows).ok()?.resolve().ok()?;
    need(export.declarations.len()==count)?;
    let limits=crate::checker::Limits::default();
    let policy=DeltaPolicy::GuardedSemanticFallback;
    let mut env=Environment::empty();
    for stage in stages {
        match stage{
            Stage::Family(cert)=>{
                let mut derivation=ClosedNonrecursiveDerivation::begin(&env);
                derivation.promote_all(&export,cert.signatures,limits.judgment_steps,policy).ok()?;
                env=derivation.finish();
                if let Some((name,reduction))=cert.reduction{
                    env=env.install_recursor_reduction(name,reduction).ok()?;
                }
            },
            Stage::Ordinary(i)=>{
                let (name,ps,ty,value,hint)=match export.declarations.get(i)?{
                    Declaration::Definition{name,level_params,ty,value,preferred_for_reduction}=>
                        (*name,level_params.clone(),*ty,*value,Some(*preferred_for_reduction)),
                    Declaration::Theorem{name,level_params,ty,value,..}=>
                        (*name,level_params.clone(),*ty,*value,None),
                    _=>return None,
                };
                let checker=TypeChecker::with_level_substitution(
                    &export.exprs,&export.levels,&env,
                    ps.iter().map(|p|(*p,LevelTerm::param(format!("u#{}",p.0)))).collect()
                ).with_delta_policy(policy);
                if hint.is_some(){need(checker.is_type(ty,limits.judgment_steps).is_proven())?;}
                else{need(checker.is_proposition(ty,limits.judgment_steps).is_proven())?;}
                let expected=TypeValue::Term(checker.closure(ty,EnvFrame::empty()));
                need(checker.check_formed(value,&expected,limits.judgment_steps).is_proven())?;
                let d=if let Some(h)=hint{ConstantDecl::definition(ps,ty,value,h)}
                else{ConstantDecl::theorem(ps,ty)};
                env=env.extend(name,d).ok()?;
            }
        }
    }
    Some(super::super::Outcome::AcceptReconstructed)
}
