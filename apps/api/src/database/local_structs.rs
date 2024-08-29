use serde::{Deserialize, Serialize};
use serde_json::Value;
use surrealdb::sql::Thing;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct User {
    name: String,
    phone_number: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct Scope {
    user: Thing,
    space: Option<Thing>,
    scope_name: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct Space {
    name: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct AcademicYear {
    year: i32,
    space: Thing,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct AcademicYearCourse {
    grade: String,
    subjects: Vec<String>,
    academic_year: Thing,
    space: Thing,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct Group {
    schedule: Vec<Value>,
    academic_year: Thing,
    space: Thing,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct Student {
    name: String,
    _name: String,
    phone_numbers: Vec<Value>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct Enrollment {
    name: String,
    _name: String,
    student: Thing,
    default_group: Thing,
    academic_year: Thing,
    course: Thing,
    space: Thing,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[serde(untagged)]
pub enum Content {
    User(User),
    Scope(Scope),
    Space(Space),
    AcademicYear(AcademicYear),
    AcademicYearCourse(AcademicYearCourse),
    Group(Group),
    Student(Student),
    Enrollment(Enrollment),
}
