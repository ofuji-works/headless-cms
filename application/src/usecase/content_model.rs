use std::sync::Arc;
use anyhow::{
    Result,
    Error,
}; 
use derive_new::new;
use serde_json::{
    Value,
    from_value,
};

use domain::{
    model::{
        content_model::ContentModel,
        field_meta::FieldMeta,
    }, repository::content_model::{
        ContentModelRepository, CreateContentModel, UpdateContentModel
    }
};

pub struct CreateContentModelInput {
    name: String,
    api_identifier: String,
    description: Option<String>,
    fields: Value, 
}

impl TryFrom<CreateContentModelInput> for CreateContentModel {
    type Error = Error; 

    fn try_from(value: CreateContentModelInput) -> Result<Self> {
       let CreateContentModelInput {
            name,
            api_identifier,
            description,
            fields,
       } = value; 

       let filed_metas: Vec<FieldMeta> = from_value(fields)?;

       Ok(Self {
            name,
            api_identifier,
            description,
            fields: filed_metas,
       })
    }
}

pub struct UpdateContentModelInput {
    id: String,
    name: Option<String>,
    api_identifier: Option<String>,
    description: Option<String>,
    fields: Option<Value>,
}

impl TryFrom<UpdateContentModelInput> for UpdateContentModel  {
    type Error = Error;
    fn try_from(input: UpdateContentModelInput) -> Result<Self> {
        let UpdateContentModelInput {
            id,
            name,
            api_identifier,
            description,
            fields,
        } = input;

        let field_metas: Option<Vec<FieldMeta>> = match fields {
            Some(input) => {
                from_value(input)?
            },
            None => None 
        };

        Ok(Self {
            id,
            name,
            api_identifier,
            description,
            fields: field_metas,
        })
    }
}

#[derive(new)]
pub struct ContentModelUsecase {
    repository: Arc<dyn ContentModelRepository>
}

impl ContentModelUsecase {
    pub async fn get(&self) -> Result<Vec<ContentModel>> {
        self.repository.clone().get().await
    }

    pub async fn create(&self, input: CreateContentModelInput) -> Result<()> {
        self.repository.clone().create(CreateContentModel::try_from(input)?).await
    }

    pub async fn update(&self, input: UpdateContentModelInput) -> Result<()> {
        self.repository.clone().update(UpdateContentModel::try_from(input)?).await
    }

    pub async fn delete(&self, id: String) -> Result<()> {
        self.repository.clone().delete(id).await
    }
}

