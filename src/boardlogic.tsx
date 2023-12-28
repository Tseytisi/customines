import { createStore, produce } from "solid-js/store";
import { createContext, batch, createSignal, useContext } from "solid-js";
import { invoke } from "@tauri-apps/api";
import { getContext, GameState } from "./customines.tsx";

const BoardLogicContext = createContext();

interface CellDetails {
    state: string,
    value: number
}

interface GameChange {
    hidden_cell_count: number,
    flag_count: number,
    mines_remaining: number,
    game_state: string,
    changes: BoardChange[]
}

interface BoardChange {
    x: number,
    y: number,
    state: string | null,
    value: number | null
}

export function BoardLogicProvider(props: any) {
    const [cellDetails, setCellDetails] = createStore<CellDetails[][]>([]);
    const [firstMove, setFirstMove] = createSignal(true);
    const app = getContext();

    // Left click action of a cell
    async function poke(x: number, y: number) {
        // Disable poking on a game-over/game-won
        if (app.gameState() != GameState.Playing && app.gameState() != GameState.BeforeGame) { return; }
        // The board should be generated _after_ the first poke
        if (firstMove()) {
            setFirstMove(false);
            await generateBoard(x, y);
            // Formally start the game
            app.setGameState(GameState.Playing);
        }
        processChanges(await invoke("poke", {
            x: x,
            y: y
        }));
    }

    // Right click action of a cell
    async function mark(x: number, y: number) {
        // Disable marking when the game hasn't started yet
        if (app.gameState() != GameState.Playing) { return; }
        processChanges(await invoke('mark', {
            x: x,
            y: y
        }));
    }

    async function generateBoard(firstX: number, firstY: number) {
        setCellDetails(await invoke("generate_board", {
            width: props.boardWidth(),
            height: props.boardHeight(),
            mines: props.mineCount(),
            firstX: firstX, 
            firstY: firstY
        }))
    }

    function processChanges(gameChanges: GameChange) {
        batch(() => {
            props.setFlagCount(gameChanges.flag_count);
            props.setHiddenCellCount(gameChanges.hidden_cell_count);
            props.setMinesRemaining(gameChanges.mines_remaining);
            switch (gameChanges.game_state) {
                case "Playing":
                    app.setGameState(GameState.Playing);
                    break;
                case "GameOver":
                    app.setGameState(GameState.GameOver);
                    break;
                case "GameWon":
                    app.setGameState(GameState.GameWon);
                    break;
            }

            // Cell changes
            gameChanges.changes.forEach(change => {
                if (change.state != null) {
                    setCellDetails(produce((cells) => cells[change.y][change.x].state = change.state!)); 
                }
                if (change.value != null) {
                    setCellDetails(produce((cells) => cells[change.y][change.x].value = change.value!));
                }
            });
        })
    }

    function getState(x: number, y: number): string {
        if (cellDetails.length > y && y >= 0) {
            if (cellDetails[y].length > x && x >= 0) {
                return cellDetails[y][x].state;
            }
        }
        return "Hidden";
    }

    function getValue(x: number, y: number): number {
        if (cellDetails.length > y && y >= 0) {
            if (cellDetails[y].length > x && x >= 0) {
                return cellDetails[y][x].value;
            }
        }
        return 0;
    }

    const functions = {
        poke: poke,
        mark: mark,
        getState: getState,
        getValue: getValue,
    }

    return <BoardLogicContext.Provider value={functions}>
        {props.children}
    </BoardLogicContext.Provider>
}

export function getLogic() {
    return useContext(BoardLogicContext);
} 