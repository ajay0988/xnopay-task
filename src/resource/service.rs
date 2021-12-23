use bson::{oid::ObjectId, Document};
use log::*;

use super::Employee;
use crate::db::*;
use crate::common::*;

// create the employee service
pub fn db_create_Employee(
    Employee: Employee
) -> Result<std::option::Option<Employee>, mongodb::error::Error> {
    let d: Document = struct_to_document(&Employee).unwrap();
    let coll = collection(Employee::COLLECTION_NAME);
    let insertion_result = coll.insert_one(d, None)?;
    let new_id: String = insertion_result.inserted_id
        .as_object_id()
        .map(ObjectId::to_hex)
        .unwrap();
    info!("save Employee, id={}", new_id);
    let res = coll.find_one(
        Some(doc! {"_id" => ObjectId::with_string(&new_id).unwrap()}),
        None
    );

    match res.unwrap() {
        None => Ok(None),
        Some(doc) => Ok(bson::from_bson(bson::Bson::Document(doc)).unwrap())
    }
}
// read one single employee details service
pub fn db_read_Employee(
    id: &str
) -> Result<std::option::Option<Employee>, mongodb::error::Error> {
    let coll = collection(Employee::COLLECTION_NAME);
    let res = coll.find_one(
        Some(doc! {"_id" => ObjectId::with_string(id).unwrap()}),
        None);
    info!("Retrieving Employee with id: {}", id);
    match res.unwrap() {
        None => Ok(None),
        Some(doc) => Ok(bson::from_bson(bson::Bson::Document(doc)).unwrap())
    }
}

// get all employee details service

pub fn db_read_all_Employees(
) -> Result<std::vec::Vec<Employee>, mongodb::error::Error> {
    let coll = collection(Employee::COLLECTION_NAME);
    let cursor = coll.find(None, None);
    let res = cursor.map(|mut x| x.as_vec::<Employee>());
    info!("Retrieving all Employee objects");
    Ok(res.unwrap())
}


// update a signle employee service
pub fn db_update_Employee(
    id: &str,
    Employee: Employee
) -> Result<std::option::Option<Employee>, mongodb::error::Error> {
    let coll = collection(Employee::COLLECTION_NAME);
    let filter = doc! {"_id" => ObjectId::with_string(id).unwrap()};
    let update_doc = doc! {"$set": struct_to_document(&Employee).unwrap()};

    let effect = coll.update_one(filter, update_doc, None);
    if effect.unwrap().modified_count < 1 {
        ()
    }

    let res = coll.find_one(
        Some(doc! {"_id" => ObjectId::with_string(id).unwrap()}),
        None
    );
    info!("Updating Employee with id: {}", id);
    match res.unwrap() {
        None => Ok(None),
        Some(doc) => Ok(bson::from_bson(bson::Bson::Document(doc)).unwrap())
    }
}
// delete a employee service
pub fn db_delete_Employee(
    id: &str
) -> Result<(), mongodb::error::Error> {
    let coll = collection(Employee::COLLECTION_NAME);
    let filter = doc! {"_id" => ObjectId::with_string(id).unwrap()};

    let effect = coll.delete_one(filter, None);
    if effect.unwrap().deleted_count < 1 {
        ()
    }
    info!("Deleting Employee with id: {}", id);
    Ok(())
}
