import { createSignal, createContext, useContext, Switch, Match } from "solid-js";
import MainMenu from "./mainmenu.tsx"
import Game from "./game.tsx"
import { invoke } from "@tauri-apps/api";

export const MAX_BOARD_HEIGHT = 100;
export const MAX_BOARD_WIDTH = 100;

const CustominesContext = createContext();

export enum Screen {
    MainMenu,
    Game
} 

export enum GameState {
    Playing,
    Paused,
    GameOver,
    GameWon,
    BeforeGame
}

export function Customines() {
    const [boardHeight, setBoardHeight] = createSignal(7);
    const [boardWidth, setBoardWidth] = createSignal(9);
    const [mineCount, setMineCount] = createSignal(10);
    const [questionsEnabled, setQuestionsEnabled] = createSignal(true);
    const [quickUncover, setQuickUncover] = createSignal(true);
    const [protectedQuickUncover, setProtectedQuickUncover] = createSignal(true);
    const settings = getContext();

    async function setGameSettings() {
        await invoke("set_game_settings", {
            settings: {
                questions_enabled: questionsEnabled(),
                quick_uncover: quickUncover(),
                protected_quick_uncover: protectedQuickUncover()
            }
        });
    }

    const gameProps = {boardHeight, boardWidth, mineCount, setGameSettings};
    const mainMenuProps = {boardHeight, boardWidth, mineCount, 
        setBoardHeight, setBoardWidth, setMineCount,
        questionsEnabled, setQuestionsEnabled,
        quickUncover, setQuickUncover,
        protectedQuickUncover, setProtectedQuickUncover
    };
    
    return (
        <div class="container">
            <Switch>
                <Match when={settings.screen() === Screen.Game}>
                    <Game {...gameProps} />
                </Match>
                <Match when={settings.screen() === Screen.MainMenu}>
                    <MainMenu {...mainMenuProps} />
                </Match>
            </Switch>
        </div>
    )
}

export function CustominesProvider(props: any) {
    const [screen, setScreen] = createSignal(Screen.MainMenu);
    const [gameState, setGameState] = createSignal(GameState.BeforeGame);
    const settings = {
        screen, setScreen,
        gameState, setGameState
    };
    return (
        <CustominesContext.Provider value={settings}>
            {props.children}
        </CustominesContext.Provider>
    )
}

export function getContext(): any { return useContext(CustominesContext); }
