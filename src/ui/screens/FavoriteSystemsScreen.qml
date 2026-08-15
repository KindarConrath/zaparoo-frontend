// Zaparoo Frontend
// Copyright (c) 2026 Wizzo Pty Ltd and the Zaparoo Project contributors.
// SPDX-License-Identifier: LicenseRef-PolyForm-Noncommercial-1.0.0

import QtQuick
import Zaparoo.Theme
import Zaparoo.Ui
import Zaparoo.Browse as Browse

// Favorite Systems screen — paged grid driven by `Browse.FavoriteSystemsModel`.
// Pure input dispatcher: emits a system id on Accept and the shared Hub signal
// on Back. Main.qml owns destination choice and transition orchestration.
MediaListScreen {
    id: favoriteSystems

    property alias favoriteSystemsGrid: favoriteSystems.mediaGrid

    signal requestAccept(string systemId)

    mediaModel: Browse.FavoriteSystemsModel
    mediaState: Browse.FavoriteSystemsState
    screenTitle: qsTr("Favorites")
    gridViewId: "systemsGrid"
    listViewId: "systemsList"
    tateListViewId: "systemsListTate"
    showTopStrip: true
    showBottomStatusRow: false
    activeLabelAtBottom: false
    gridBottomMargin: Sizing.pctH(8) + Sizing.pctH(7)
    topStripTitleProvider: () => qsTr("Favorites")
    topStripTotalTextProvider: () => favoriteSystems.mediaGrid.itemCount > 0 ? qsTr("%1 systems").arg(Browse.FavoriteSystemsModel.count) : ""
    topStripRightTextProvider: () => !favoriteSystems._listLayout || favoriteSystems.mediaGrid.itemCount <= 0 ? "" : qsTr("%1 / %2").arg(favoriteSystems.mediaGrid.currentIndex + 1).arg(Math.max(1, Browse.FavoriteSystemsModel.count))
    activeLabelTextProvider: () => favoriteSystems.mediaGrid.itemCount > 0 ? Browse.FavoriteSystemsModel.name_at(favoriteSystems.mediaGrid.currentIndex) : ""
    activeLabelTagsProvider: () => {
        if (favoriteSystems.mediaGrid.itemCount <= 0)
            return "";
        const systemId = Browse.FavoriteSystemsModel.system_id_at(favoriteSystems.mediaGrid.currentIndex);
        const count = Browse.FavoriteSystemsModel.media_count_for_system(systemId);
        return count >= 0 ? qsTr("%1 favorites").arg(count) : "";
    }
    gridColumnsOverride: Sizing.systemsGridShape(Sizing.screenWidth, Sizing.screenHeight).columns
    gridRowsOverride: Sizing.systemsGridShape(Sizing.screenWidth, Sizing.screenHeight).rows
    emptyText: qsTr("No favorites yet")
    loadingText: qsTr("Loading favorite systems…")
    detailShowTitle: false
    detailShowDescription: false
    detailPlaceholderKey: "icons/Console"
    pageMenuEnabledWhenEmpty: true
    retryAction: () => Browse.FavoriteSystemsModel.retry()

    acceptAction: index => {
        if (favoriteSystems.mediaModel === null)
            return;
        if (favoriteSystems.mediaGrid.itemCount <= 0)
            return;
        const systemId = Browse.FavoriteSystemsModel.system_id_at(index);
        favoriteSystems.requestAccept(systemId);
    }
}
