#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::{
    models::{
        _entities::sea_orm_active_enums::Role,
        join::user_pack::load_user_and_one_pack,
        packs::CreatePackPayload,
        packs_translations::{
            AdminPackTranslatedPayload, PackTranslationModelList, TranslateGroupView,
        },
        PackTranslationModel,
    },
    views::{self},
};
use axum::{debug_handler, response::Redirect};
use loco_rs::prelude::*;
use routes::AdminRoutes;

use crate::models::{packs::PackModelList, users::UserPid, PackModel, UserModel};

pub mod routes {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct AdminRoutes {
        pub base: String,
        pub admin_packs: String,
        pub admin_packs_img: String,
        pub admin_packs_add: String,
        pub admin_packs_edit: String,
        pub admin_packs_add_img: String,
    }
    impl AdminRoutes {
        pub fn init() -> Self {
            Self {
                base: String::from(Admin::BASE),
                admin_packs: format!("{}{}", Admin::BASE, Admin::ADMIN_PACKS),
                admin_packs_img: format!("{}{}", Admin::BASE, Admin::ADMIN_PACKS_IMG),
                admin_packs_add: format!("{}{}", Admin::BASE, Admin::ADMIN_PACK_ADD),
                admin_packs_edit: format!("{}{}", Admin::BASE, Admin::ADMIN_PACK_EDIT),
                admin_packs_add_img: format!("{}{}", Admin::BASE, Admin::ADMIN_PACK_ADD_IMG),
            }
        }
    }

    #[derive(Clone, Debug, Serialize)]
    pub struct Admin;
    impl Admin {
        pub const BASE: &'static str = "/admin";
        pub const ADMIN_PACKS: &'static str = "/packs";
        pub const ADMIN_PACKS_IMG: &'static str = "/packs/img";
        pub const ADMIN_PACK_ADD: &'static str = "/pack/add";
        pub const ADMIN_PACK_ADD_TRANSLATE: &'static str = "/pack/add/translate";
        pub const ADMIN_PACK_EDIT_ID: &'static str = "/pack/edit/{pid}";
        pub const ADMIN_PACK_EDIT_ID_TRANSLATE: &'static str = "/pack/edit/translate";
        pub const ADMIN_PACK_EDIT: &'static str = "/pack/edit";
        pub const ADMIN_PACK_ADD_IMG: &'static str = "/pack/add/img";
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("admin")
        .add(routes::Admin::ADMIN_PACKS, get(admin_packs))
        .add(routes::Admin::ADMIN_PACKS_IMG, get(admin_packs_img))
        .add(routes::Admin::ADMIN_PACK_ADD, post(add_pack))
        .add(
            routes::Admin::ADMIN_PACK_ADD_TRANSLATE,
            post(add_pack_translate),
        )
        .add(routes::Admin::ADMIN_PACK_EDIT_ID, get(edit_pack_view))
        .add(
            routes::Admin::ADMIN_PACK_EDIT_ID_TRANSLATE,
            post(edit_pack_translate),
        )
        .add(routes::Admin::ADMIN_PACK_EDIT_ID, post(edit_pack))
        .add(routes::Admin::ADMIN_PACK_ADD_IMG, post(admin_packs_img))
}

pub async fn load_user(db: &DatabaseConnection, user_pid: &UserPid) -> Result<UserModel> {
    let item = UserModel::find_by_pid(db, user_pid.as_ref()).await?;
    Ok(item)
}
async fn load_packs(db: &DatabaseConnection) -> Result<PackModelList> {
    let list = PackModel::find_all_packs(db).await?;
    Ok(PackModelList::new(list))
}
async fn load_pack_one(db: &DatabaseConnection, pid: &Uuid) -> Result<PackModel> {
    let pack: crate::models::packs::Model = PackModel::find_by_pid(db, pid).await?;
    Ok(pack)
}
async fn load_pack_one_by_id(db: &DatabaseConnection, id: &i32) -> Result<PackModel> {
    let pack: crate::models::packs::Model = PackModel::find_by_id(db, id).await?;
    Ok(pack)
}
async fn load_pack_translated_by_pack_id(
    db: &DatabaseConnection,
    id: &i32,
) -> Result<PackTranslationModelList> {
    let pack = PackTranslationModel::find_by_all_pack_id(db, id).await?;
    Ok(pack)
}

#[debug_handler]
pub async fn edit_pack_translate(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
    Json(form): Json<AdminPackTranslatedPayload>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let user = load_user(&ctx.db, &user_pid).await?;
    if user.role != Role::Admin {
        return Ok(Redirect::to("/login").into_response());
    }
    dbg!(&form);
    let language = form.language.clone();
    form.update(&ctx.db).await?;
    let pack = load_pack_one_by_id(&ctx.db, &form.pack_id).await?;
    let admin_routes = AdminRoutes::init();
    let is_successfully_updated = true;
    let view_output = views::admin::packs_form_edit_partial_translated(
        v,
        &form.into(),
        &user.into(),
        &admin_routes,
        &language,
        &pack.into(),
        is_successfully_updated,
    )?;
    Ok(view_output.into_response())
}

#[debug_handler]
pub async fn edit_pack(
    auth: auth::JWT,
    Path(pack_pid): Path<Uuid>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
    Json(mut form): Json<CreatePackPayload>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let (user, pack) = load_user_and_one_pack(&ctx.db, &user_pid, &pack_pid).await?;
    if user.role != Role::Admin {
        return Ok(Redirect::to("/login").into_response());
    }
    dbg!(&form);
    form.pid = pack_pid.clone();
    form.sanitize_title_url_in_place();
    let _ = pack.update_pack_admin(&form, &ctx.db).await?;
    let packs = load_packs(&ctx.db).await?;
    let admin_routes = AdminRoutes::init();
    let view_output =
        views::admin::packs_form_partial(v, &packs.into(), &user.into(), false, &admin_routes)?;
    Ok(view_output.into_response())
}

#[debug_handler]
pub async fn edit_pack_view(
    auth: auth::JWT,
    Path(pid): Path<Uuid>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let user = load_user(&ctx.db, &user_pid).await?;
    if user.role != Role::Admin {
        return Ok(Redirect::to("/login").into_response());
    }
    let pack = load_pack_one(&ctx.db, &pid).await?;
    let pack_translates = load_pack_translated_by_pack_id(&ctx.db, &pack.id)
        .await?
        .group();
    let admin_routes = AdminRoutes::init();
    let view_output = views::admin::packs_form_edit_partial(
        v,
        &pack.into(),
        &user.into(),
        &admin_routes,
        &pack_translates,
    )?;
    Ok(view_output.into_response())
}

#[debug_handler]
pub async fn admin_packs(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let user = load_user(&ctx.db, &user_pid).await?;
    if user.role != Role::Admin {
        return Ok(Redirect::to("/login").into_response());
    }
    let packs = load_packs(&ctx.db).await?;
    let admin_routes = AdminRoutes::init();
    let pack_translates = TranslateGroupView::default();
    let view_output = views::admin::packs(
        v,
        &packs.into(),
        &user.into(),
        false,
        &admin_routes,
        &pack_translates,
    )?;
    Ok(view_output.into_response())
}
#[debug_handler]
pub async fn admin_packs_img(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let user = load_user(&ctx.db, &user_pid).await?;
    if user.role != Role::Admin {
        return Ok(Redirect::to("/login").into_response());
    }
    let packs = load_packs(&ctx.db).await?;
    let admin_routes = AdminRoutes::init();
    let pack_translates = TranslateGroupView::default();
    let view_output = views::admin::packs(
        v,
        &packs.into(),
        &user.into(),
        true,
        &admin_routes,
        &pack_translates,
    )?;
    Ok(view_output.into_response())
}

#[debug_handler]
pub async fn add_pack_translate(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
    Json(form): Json<AdminPackTranslatedPayload>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let user = load_user(&ctx.db, &user_pid).await?;
    if user.role != Role::Admin {
        return Ok(Redirect::to("/login").into_response());
    }

    dbg!(&form);
    let language = form.language.clone();
    // let form = form.save(&ctx.db).await?;
    let pack: crate::models::packs::Model = load_pack_one_by_id(&ctx.db, &form.pack_id).await?;
    let admin_routes = AdminRoutes::init();
    let is_successfully_updated = false;
    let view_output = views::admin::packs_form_edit_partial_translated(
        v,
        &form.into(),
        &user.into(),
        &admin_routes,
        &language,
        &pack.into(),
        is_successfully_updated,
    )?;
    Ok(view_output.into_response())
}

#[debug_handler]
pub async fn add_pack(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
    Json(mut form): Json<CreatePackPayload>,
) -> Result<impl IntoResponse> {
    let user_pid = UserPid::new(&auth.claims.pid);
    let user = load_user(&ctx.db, &user_pid).await?;
    if user.role != Role::Admin {
        return Ok(Redirect::to("/login").into_response());
    }
    dbg!(&form);
    form.sanitize_title_url_in_place();
    form.save(&ctx.db).await?;
    let packs = load_packs(&ctx.db).await?;
    let admin_routes = AdminRoutes::init();
    let view_output =
        views::admin::packs_form_partial(v, &packs.into(), &user.into(), false, &admin_routes)?;
    Ok(view_output.into_response())
}
