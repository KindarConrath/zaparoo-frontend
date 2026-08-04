// Zaparoo Frontend
// Copyright (c) 2026 Wizzo Pty Ltd and the Zaparoo Project contributors.
// SPDX-License-Identifier: LicenseRef-PolyForm-Noncommercial-1.0.0

import QtQuick
import Zaparoo.Theme
import Zaparoo.Ui
import Zaparoo.Browse as Browse

// Favorite Systems screen — paged grid driven by `Browse.FavoriteSystemsModel`.
// Pure input dispatcher: emits `requestHubScreen()` on Escape and
// `requestFavoritesScreenForSystem(systemId)` on Accept. It reuses the
// shared `MediaListScreen` shell to keep list/detail layout, selection
// persistence, and input plumbing consistent with Favorites and Recents.
MediaListScreen {
    id: favoriteSystems

    property alias favoriteSystemsGrid: favoriteSystems.mediaGrid

    property string selectedSystemId: ""

    mediaModel: Browse.FavoriteSystemsModel
    mediaState: Browse.FavoriteSystemsState
    screenTitle: qsTr("Favorite Systems")
    gridViewId: "systemsGrid"
    listViewId: "systemsList"
    tateListViewId: "systemsListTate"
    showTopStrip: true
    showBottomStatusRow: false
    activeLabelAtBottom: false
    gridBottomMargin: Sizing.pctH(8) + Sizing.pctH(7)
    topStripTitleProvider: () => qsTr("Favorite Systems")
    topStripTotalTextProvider: () => favoriteSystems.mediaGrid.itemCount > 0 ? qsTr("%1 systems").arg(Browse.FavoriteSystemsModel.count) : ""
    topStripRightTextProvider: () => !favoriteSystems._listLayout || favoriteSystems.mediaGrid.itemCount <= 0 ? "" : qsTr("%1 / %2").arg(favoriteSystems.mediaGrid.currentIndex + 1).arg(Math.max(1, Browse.FavoriteSystemsModel.count))
    activeLabelTextProvider: () => favoriteSystems.mediaGrid.itemCount > 0 ? Browse.FavoriteSystemsModel.name_at(favoriteSystems.mediaGrid.currentIndex) : ""
    activeLabelTagsProvider: () => ""
    gridColumnsOverride: Sizing.systemsGridShape(Sizing.screenWidth, Sizing.screenHeight).columns
    gridRowsOverride: Sizing.systemsGridShape(Sizing.screenWidth, Sizing.screenHeight).rows
    emptyText: qsTr("No favorited systems yet")
    loadingText: qsTr("Loading favorite systems…")
    detailShowTitle: false
    detailShowDescription: false
    detailPlaceholderKey: "icons/Console"

    acceptAction: index => {
        if (favoriteSystems.mediaModel === null)
            return;
        if (favoriteSystems.mediaGrid.itemCount <= 0)
            return;
        const systemId = Browse.FavoriteSystemsModel.system_id_at(index);
        favoriteSystems.requestFavoritesScreenForSystem(systemId);
    }

    cancelAction: () => {
        favoriteSystems.requestHubScreen();
    }

    onListLayoutEntered: () => {
        if (typeof favoriteSystems.mediaModel.ensure_loaded === "function")
            Browse.FavoriteSystemsModel.ensure_loaded();
    }

    signal requestFavoritesScreenForSystem(string systemId)
}
