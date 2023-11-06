use std::path::PathBuf;

mod files;
mod specification;
pub use specification::*;

fn get_document(
    project: PathBuf,
    type_: DocumentType,
    errors: &mut Vec<String>,
) -> Option<Document> {
    let path = project.join(type_.file_name());

    let content = match files::read_file(path) {
        Ok(content) => content,
        Err(error) => {
            errors.push(error);
            return None;
        }
    };
    match Document::try_new(content, type_) {
        Ok(document) => Some(document),
        Err(Error(new_errors)) => {
            errors.extend(new_errors);
            None
        }
    }
}

/// Returns the set of all documents defining the software's specification,
/// as specified in
pub fn get_documents(project: PathBuf) -> Result<Documents, Error> {
    let mut errors = vec![];

    let requirements = get_specification(project.clone(), &mut errors);
    let design = get_document(project.clone(), DocumentType::Design, &mut errors);
    let risk_assessment = get_document(project.clone(), DocumentType::Risks, &mut errors);
    let verification_plan = get_document(project.clone(), DocumentType::Tests, &mut errors);
    let user_manual = get_document(project.clone(), DocumentType::UserManual, &mut errors);
    let operator_manual = get_document(project.clone(), DocumentType::OperatorManual, &mut errors);
    let retirement_plan = get_document(project, DocumentType::RetirementPlan, &mut errors);

    if errors.is_empty() {
        Documents::try_new(
            requirements,
            design.unwrap(),
            risk_assessment.unwrap(),
            verification_plan.unwrap(),
            user_manual.unwrap(),
            operator_manual.unwrap(),
            retirement_plan.unwrap(),
        )
    } else {
        Err(Error(errors))
    }
}
