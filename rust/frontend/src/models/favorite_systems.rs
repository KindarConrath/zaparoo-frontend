// Zaparoo Frontend
// Copyright (c) 2026 Wizzo Pty Ltd and the Zaparoo Project contributors.
// SPDX-License-Identifier: LicenseRef-PolyForm-Noncommercial-1.0.0
//
// `Browse.FavoriteSystemsModel` — a flat systems list driven by the
// filtered systems-favorites resource.

use crate::image_overrides;
use crate::models::{with_hidden_browse_prefs_read, with_persist_read};
use crate::system_region::Region;
use crate::{system_logos, system_name_overrides, system_names, system_region};
use cxx_qt::CxxQtType;
use cxx_qt_lib::{QByteArray, QHash, QHashPair_i32_QByteArray, QModelIndex, QString, QVariant};
use std::pin::Pin;
use zaparoo_core::endpoints::systems_favorites::SystemsFavoritesEndpoint;
use zaparoo_core::media_types::SystemsResult;
use zaparoo_core::remote_resource::ResourceStatus;

use crate::models::systems::SystemInfo;

const COVER_KEY_ROLE: i32 = 256 + 1;
const NAME_ROLE: i32 = 256 + 2;
const FAVORITE_ROLE: i32 = 256 + 3;
const FILE_STEM_ROLE: i32 = 256 + 4;
const HIDDEN_ROLE: i32 = 256 + 5;
const DISAMBIGUATING_TAGS_ROLE: i32 = 256 + 6;

#[derive(Default)]
pub struct FavoriteSystemsModelRust {
    systems: Vec<SystemInfo>,
    count: i32,
    loading: bool,
    cover_requests_paused: bool,
    error_message: QString,
    current_detail_image_key: QString,
    current_detail_tags: QString,
    current_detail_loading: bool,
    detail_prefetch_key_next: QString,
    detail_prefetch_key_prev: QString,
}

#[cxx_qt::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("model_includes.h");

        #[allow(non_snake_case, reason = "Qt class names are PascalCase")]
        type QAbstractListModel;

        type QModelIndex = cxx_qt_lib::QModelIndex;
        type QVariant = cxx_qt_lib::QVariant;
        type QHash_i32_QByteArray = cxx_qt_lib::QHash<cxx_qt_lib::QHashPair_i32_QByteArray>;
        type QByteArray = cxx_qt_lib::QByteArray;
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[base = QAbstractListModel]
        #[qml_element]
        #[qml_singleton]
        #[qproperty(i32, count)]
        #[qproperty(bool, loading)]
        #[qproperty(bool, cover_requests_paused)]
        #[qproperty(QString, error_message)]
        #[qproperty(QString, current_detail_image_key)]
        #[qproperty(QString, current_detail_tags)]
        #[qproperty(bool, current_detail_loading)]
        #[qproperty(QString, detail_prefetch_key_next)]
        #[qproperty(QString, detail_prefetch_key_prev)]
        type FavoriteSystemsModel = super::FavoriteSystemsModelRust;

        #[qinvokable]
        fn fetch_more(self: Pin<&mut FavoriteSystemsModel>);

        #[qinvokable]
        fn name_at(self: &FavoriteSystemsModel, index: i32) -> QString;

        #[qinvokable]
        fn path_at(self: &FavoriteSystemsModel, index: i32) -> QString;

        #[qinvokable]
        fn system_id_at(self: &FavoriteSystemsModel, index: i32) -> QString;

        #[qinvokable]
        fn index_for_path(self: &FavoriteSystemsModel, path: &QString) -> i32;

        #[qinvokable]
        fn disambiguating_tags_at(self: &FavoriteSystemsModel, index: i32) -> QString;

        #[qinvokable]
        fn clear_current_detail(self: Pin<&mut FavoriteSystemsModel>);

        #[qinvokable]
        fn peek_detail_at(self: Pin<&mut FavoriteSystemsModel>, index: i32);

        #[qinvokable]
        fn load_detail_at(self: Pin<&mut FavoriteSystemsModel>, index: i32);

        #[qinvokable]
        fn refresh_cover_keys(self: Pin<&mut FavoriteSystemsModel>, first_row: i32, count: i32);

        #[qinvokable]
        fn clear_pending_cover_requests(self: Pin<&mut FavoriteSystemsModel>);

        #[inherit]
        #[cxx_name = "beginResetModel"]
        fn begin_reset_model(self: Pin<&mut FavoriteSystemsModel>);

        #[inherit]
        #[cxx_name = "endResetModel"]
        fn end_reset_model(self: Pin<&mut FavoriteSystemsModel>);

        #[cxx_name = "rowCount"]
        fn row_count(self: &FavoriteSystemsModel, parent: &QModelIndex) -> i32;
        fn data(self: &FavoriteSystemsModel, index: &QModelIndex, role: i32) -> QVariant;
        #[cxx_name = "roleNames"]
        fn role_names(self: &FavoriteSystemsModel) -> QHash_i32_QByteArray;
    }

    impl cxx_qt::Threading for FavoriteSystemsModel {}
    impl cxx_qt::Initialize for FavoriteSystemsModel {}
}

crate::bind_to_endpoint! {
    for ffi::FavoriteSystemsModel,
    endpoint = SystemsFavoritesEndpoint,
    args = (),
    select = project,
    apply = apply_state,
}

fn project(status: &ResourceStatus<SystemsResult>) -> (Option<SystemsResult>, String, bool) {
    match status {
        ResourceStatus::Ready(data) => (Some(data.clone()), String::new(), false),
        ResourceStatus::Errored { message, .. } => (None, message.clone(), false),
        ResourceStatus::Idle | ResourceStatus::Loading => (None, String::new(), true),
    }
}

fn rows_for_catalog(
    catalog: Option<&SystemsResult>,
    hidden_ids: &[String],
    show_hidden: bool,
    region: Region,
) -> Vec<SystemInfo> {
    catalog.map_or_else(Vec::new, |c| {
        c.systems
            .iter()
            .filter_map(|s| {
                let is_hidden = hidden_ids.contains(&s.id);
                if is_hidden && !show_hidden {
                    return None;
                }
                let name = system_name_overrides::lookup(&s.id)
                    .or_else(|| system_names::localized_name(&s.id, region))
                    .unwrap_or_else(|| s.name.clone());
                let cover_key = image_overrides::override_path("systems", &s.id).map_or_else(
                    || format!("systems/{}", system_logos::logo_artwork_stem(&s.id, region)),
                    |p| format!("custom-image/{}", p.display()),
                );
                Some(SystemInfo {
                    id: s.id.clone(),
                    name,
                    cover_key,
                    category: s.category.clone(),
                    release_date: s.release_date.clone(),
                    manufacturer: s.manufacturer.clone(),
                    hidden: is_hidden,
                    zap_script: s.zap_script.clone(),
                })
            })
            .collect()
    })
}

fn apply_state(
    mut model: Pin<&mut ffi::FavoriteSystemsModel>,
    (data, err, is_loading): (Option<SystemsResult>, String, bool),
) {
    if let Some(data) = data {
        let hidden_ids = with_hidden_browse_prefs_read(|p| p.hidden_system_ids.clone());
        let show_hidden = with_persist_read(|s| s.settings.show_hidden);
        let region = system_region::current_region();
        let rows = rows_for_catalog(Some(&data), &hidden_ids, show_hidden, region);
        let count = rows.len() as i32;
        model.as_mut().begin_reset_model();
        model.as_mut().rust_mut().systems = rows;
        model.as_mut().rust_mut().count = count;
        model.as_mut().end_reset_model();
        model.as_mut().count_changed();
        if model.loading {
            model.as_mut().set_loading(false);
        }
    }

    let qerr = QString::from(err.as_str());
    if model.error_message != qerr {
        model.as_mut().set_error_message(qerr);
    }

    if !err.is_empty() && model.loading {
        model.as_mut().set_loading(false);
    } else if model.loading != is_loading {
        model.as_mut().set_loading(is_loading);
    }
}

impl ffi::FavoriteSystemsModel {
    fn row_count(&self, parent: &QModelIndex) -> i32 {
        if parent.is_valid() {
            0
        } else {
            self.count
        }
    }

    fn data(&self, index: &QModelIndex, role: i32) -> QVariant {
        if !index.is_valid() || index.row() < 0 || index.row() >= self.count {
            return QVariant::default();
        }
        let s = &self.systems[index.row() as usize];
        match role {
            COVER_KEY_ROLE => QVariant::from(&QString::from(s.cover_key.as_str())),
            NAME_ROLE | FILE_STEM_ROLE => QVariant::from(&QString::from(s.name.as_str())),
            FAVORITE_ROLE => QVariant::from(&0_i32),
            HIDDEN_ROLE => QVariant::from(&s.hidden),
            DISAMBIGUATING_TAGS_ROLE => QVariant::from(&QString::default()),
            _ => QVariant::default(),
        }
    }

    fn role_names(&self) -> QHash<QHashPair_i32_QByteArray> {
        let mut h = QHash::<QHashPair_i32_QByteArray>::default();
        h.insert(COVER_KEY_ROLE, QByteArray::from("coverKey"));
        h.insert(NAME_ROLE, QByteArray::from("name"));
        h.insert(FAVORITE_ROLE, QByteArray::from("favorite"));
        h.insert(FILE_STEM_ROLE, QByteArray::from("fileStem"));
        h.insert(HIDDEN_ROLE, QByteArray::from("hidden"));
        h.insert(
            DISAMBIGUATING_TAGS_ROLE,
            QByteArray::from("disambiguatingTags"),
        );
        h
    }

    fn fetch_more(self: Pin<&mut Self>) {
        // All systems are loaded in one pass from CatalogEndpoint. No paging.
    }

    fn clear_current_detail(mut self: Pin<&mut Self>) {
        self.as_mut().set_current_detail_loading(false);
        self.as_mut()
            .set_current_detail_image_key(QString::default());
        self.as_mut().set_current_detail_tags(QString::default());
        self.as_mut()
            .set_detail_prefetch_key_next(QString::default());
        self.as_mut()
            .set_detail_prefetch_key_prev(QString::default());
    }

    fn peek_detail_at(mut self: Pin<&mut Self>, index: i32) {
        self.as_mut().load_detail_at(index);
    }

    fn load_detail_at(mut self: Pin<&mut Self>, _index: i32) {
        self.as_mut().set_current_detail_loading(false);
        self.as_mut()
            .set_current_detail_image_key(QString::default());
        self.as_mut().set_current_detail_tags(QString::default());
    }

    fn refresh_cover_keys(self: Pin<&mut Self>, _first_row: i32, _count: i32) {}

    fn clear_pending_cover_requests(self: Pin<&mut Self>) {}

    fn name_at(&self, index: i32) -> QString {
        if index < 0 || index >= self.count {
            return QString::default();
        }
        QString::from(self.systems[index as usize].name.as_str())
    }

    fn path_at(&self, index: i32) -> QString {
        if index < 0 || index >= self.count {
            return QString::default();
        }
        QString::from(self.systems[index as usize].id.as_str())
    }

    fn system_id_at(&self, index: i32) -> QString {
        if index < 0 || index >= self.count {
            return QString::default();
        }
        QString::from(self.systems[index as usize].id.as_str())
    }

    fn index_for_path(&self, path: &QString) -> i32 {
        let needle = path.to_string();
        if needle.is_empty() {
            return -1;
        }
        self.systems
            .iter()
            .position(|s| s.id == needle)
            .map_or(-1, |i| i as i32)
    }

    fn disambiguating_tags_at(&self, _index: i32) -> QString {
        QString::default()
    }
}
