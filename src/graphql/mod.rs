mod person_queries;
mod user_mutations;
mod user_queries;
mod person_mutations;

use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct QueryRoot(
    user_queries::UserQueries,
    person_queries::PersonQueries);

#[derive(MergedObject, Default)]
pub struct MutationRoot(
    user_mutations::UserMutations,
    person_mutations::PersonMutations);
