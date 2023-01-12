use async_graphql::{dynamic::*, Value};
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{self, IntoResponse},
    Router,
    routing::get, Server,
};

#[tokio::main]
async fn main() {
    let my_input = InputObject::new("MyInput")
        .field(InputValue::new("a", TypeRef::named_nn(TypeRef::FLOAT)))
        .field(InputValue::new("b", TypeRef::named_nn(TypeRef::FLOAT)));

    let query = Object::new("Query").field(
        Field::new("value", TypeRef::named_nn(TypeRef::FLOAT), |ctx| {
            FieldFuture::new(async move {
                let value = ctx.args.try_get("arg")?.object()?.try_get("a")?.f64()?;
                Ok(Some(Value::from(value)))
            })
        })
            .argument(InputValue::new(
                "arg",
                TypeRef::named_nn(my_input.type_name()),
            )),
    );

    let schema = Schema::build("Query", None, None)
        .register(query)
        .register(my_input)
        .finish()
        .unwrap();

    let app = Router::new()
        .route("/", get(playground).post(graphql_handler))
        .layer(Extension(schema));

    println!("GraphiQL IDE: http://localhost:8000");

    Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn graphql_handler(schema: Extension<Schema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn playground() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

