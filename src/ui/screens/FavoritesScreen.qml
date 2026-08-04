// Zaparoo Frontend
// Copyright (c) 2026 Wizzo Pty Ltd and the Zaparoo Project contributors.
// SPDX-License-Identifier: LicenseRef-PolyForm-Noncommercial-1.0.0

import QtQuick
import Zaparoo.Browse as Browse

// Favorites screen — flat paged grid driven by
// `Browse.FavoritesModel`. Pure input dispatcher: emits
// `requestHubScreen()` on Escape and launches the highlighted entry on
// Accept by calling the model's `launch_at` (which fans out to Core's
// `run` endpoint). When opened from Favorite Systems it also scopes
// the list to the selected console id.
//
// Favorites is a flat list — no folder navigation, no card-write flow —
// so it reuses the shared `MediaListScreen` shell with the
// favorites-specific model, persisted selection state, and copy.
MediaListScreen {
    id: favorites

    property alias favoritesGrid: favorites.mediaGrid

    property string selectedSystemId: ""

    signal requestFavoriteSystemsScreen()

    cancelAction: () => {
        if (Browse.Settings.favorites_grouped)
            favorites.requestFavoriteSystemsScreen();
        else
            favorites.requestHubScreen();
    }

    mediaModel: Browse.FavoritesModel
    mediaState: Browse.FavoritesState
    screenTitle: qsTr("Favorites")
    emptyText: qsTr("No favorites yet")
    loadingText: qsTr("Loading favorites…")
    totalItemsOverride: Browse.FavoritesModel.total_items
    gridTotalItemsOverride: Browse.FavoritesModel.total_items
    gridHasMorePages: Browse.FavoritesModel.has_next_page
    topStripTotalPagesProvider: () => favorites.mediaGrid.totalPageCount
    topStripTotalTextProvider: () => Browse.FavoritesModel.total_items >= 0 ? qsTr("%1 favorites").arg(Browse.FavoritesModel.total_items) : ""
    detailShowTitle: false

    onSelectedSystemIdChanged: {
        if (typeof favorites.mediaModel.set_system === "function")
            Browse.FavoritesModel.set_system(favorites.selectedSystemId);
        else
            console.warn("favorites/qml selectedSystemIdChanged skipped set_system", "selectedSystemId=" + favorites.selectedSystemId);
    }

    Component.onCompleted: {
        if (typeof favorites.mediaModel.set_system === "function")
            Browse.FavoritesModel.set_system(favorites.selectedSystemId);
        else
            console.warn("favorites/qml componentCompleted skipped set_system", "selectedSystemId=" + favorites.selectedSystemId);
    }

    onListLayoutEntered: () => {
        if (typeof favorites.mediaModel.set_system === "function")
            Browse.FavoritesModel.set_system(favorites.selectedSystemId);
        else
            console.warn("favorites/qml listLayoutEntered skipped set_system", "selectedSystemId=" + favorites.selectedSystemId);
    }
}
