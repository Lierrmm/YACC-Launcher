import { get, writable } from 'svelte/store';

import type { GameInstall } from '../utils/GameInstall';
import { InstallType } from '..//utils/InstallType';
import { Store } from 'tauri-plugin-store-api';
import { YaccState } from '..//utils/YaccState';
import { appDir } from '@tauri-apps/api/path';
import { invoke } from "@tauri-apps/api";
import { open } from '@tauri-apps/api/dialog';

const persistentStore = new Store('yacc-launcher-settings.json');

export interface YaccLauncherStore {
    developer_mode: boolean,
    game_path: string,
    install_type: InstallType,

    launcher_version: string,

    installed_yacc_version: string,
    yacc_state: YaccState,
    
    yacc_is_running: boolean,

    player_count: number,
    server_count: number,
}

const defaultSettings: YaccLauncherStore = {
    developer_mode: false,
    game_path: undefined as unknown as string,
    install_type: undefined as unknown as InstallType,
    launcher_version: "",
    installed_yacc_version: "",
    yacc_state: YaccState.GAME_NOT_FOUND,
    yacc_is_running: false,
    player_count: -1,
    server_count: -1
};

const _store = () => {

    const { subscribe, update } = writable(defaultSettings);

    const methods = {
        async toggleDevMode () {
            let currentStore: YaccLauncherStore = get(this);
            update(state => ({...state, developer_mode: !state.developer_mode}));
            await persistentStore.set('dev_mode', currentStore.developer_mode);
            await persistentStore.save();
        },
        async updateGamePath () {
            // Open a selection dialog for directories
            const selected = await open({
                directory: true,
                multiple: false,
                defaultPath: await appDir(),
            });
            if (Array.isArray(selected)) {
                // user selected multiple directories
                alert("Please only select a single directory");
            } else if (selected === null) {
                // user cancelled the selection
            } else {
                // user selected a single directory

                // Verify if valid COD4 install location
                let is_valid_cod4_install = await invoke("verify_install_location", { gamePath: selected }) as boolean;
                if (is_valid_cod4_install) {
                    update(state => ({ ...state, game_path: selected, install_type: InstallType.STEAM, yacc_state: YaccState.READY_TO_PLAY}));
                    // showNotification(
                    //     i18n.global.tc('notification.game_folder.new.title'),
                    //     i18n.global.tc('notification.game_folder.new.text')
                    // );
                    // try {
                    //     notification_handle.close();
                    // }
                    // catch {
                    //     console.warn("Nothing to close");
                    // }

                    let game_install = {
                        game_path: selected,
                        install_type: InstallType.STEAM
                    } as GameInstall;

                    // Save change in persistent store
                    await persistentStore.set('game-install', { value: game_install });
                    await persistentStore.save(); // explicit save to disk

                    // Check for YACC install
                    // store.commit('checkNorthstarUpdates');
                }
                else {
                    // Not valid COD4 install
                    // showErrorNotification(
                    //     i18n.global.tc('notification.game_folder.wrong.text'),
                    //     i18n.global.tc('notification.game_folder.wrong.title')
                    // );
                    console.error("Not a valid COD4 install location");
                }
            }
        },
        async LaunchGame (no_checks = false) {

            let currentStore: YaccLauncherStore = get(this);
            let game_install = {
                game_path: currentStore.game_path,
                install_type: currentStore.install_type
            } as GameInstall;

            if (no_checks) {
                await invoke("launch_yacc", { gameInstall: game_install, bypassChecks: no_checks })
                    .then((message) => {
                        console.log("Launched with bypassed checks");
                        console.log(message);
                    })
                    .catch((error) => {
                        console.error(error);
                        alert(error);
                    });

                return;
            }

            // TODO update installation if release track was switched
            switch (currentStore.yacc_state) {
                // Install yacc if it wasn't detected.
                case YaccState.INSTALL:
                    let install_yacc_result = invoke("install_yacc_caller", { gamePath: currentStore.game_path });
                    currentStore.yacc_state = YaccState.INSTALLING;

                    await install_yacc_result.then((message) => {
                        console.log(message);
                    })
                        .catch((error) => {
                            console.error(error);
                            alert(error);
                        });

                    // _get_northstar_version_number(state);
                    break;

                // Update yacc if it is outdated.
                case YaccState.MUST_UPDATE:
                    // Updating is the same as installing, simply overwrites the existing files
                    let reinstall_yacc_result = invoke("install_yacc_caller", { gamePath: currentStore.game_path });
                    currentStore.yacc_state = YaccState.UPDATING;

                    await reinstall_yacc_result.then((message) => {
                        console.log(message);
                    })
                        .catch((error) => {
                            console.error(error);
                            alert(error);
                        });

                    // _get_northstar_version_number(state);
                    break;

                // Game is ready to play.
                case YaccState.READY_TO_PLAY:
                    await invoke("launch_yacc", { gameInstall: game_install })
                        .then((message) => {
                            console.log(message);
                            // NorthstarState.RUNNING
                        })
                        .catch((error) => {
                            console.error(error);
                            // showErrorNotification(error);
                        });
                    break;

                case YaccState.GAME_NOT_FOUND:
                    // store.commit('updateGamePath');
                    console.log("Game not found!");
                    break;
            }
        }
    }
    
    return {
        subscribe,
        ...methods
    }
}

export default _store();