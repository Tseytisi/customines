import { createSignal, mergeProps, onCleanup, onMount, splitProps, Switch, Match } from "solid-js";
import Board from "./board.tsx";
import { GameState, getContext, Screen } from "./customines.tsx";

export default function Game(props: any) {
    const [someboardprops, other] = splitProps(props, ["boardHeight", "boardWidth", "mineCount"]);
    const [hiddenCellCount, setHiddenCellCount] = createSignal(props.boardWidth() * props.boardHeight());
    const [flagCount, setFlagCount] = createSignal(0);
    const [minesRemaining, setMinesRemaining] = createSignal(props.mineCount());
    const boardprops = mergeProps(someboardprops, {hiddenCellCount, setHiddenCellCount, 
        flagCount, setFlagCount, minesRemaining, setMinesRemaining});
    const app = getContext();

    const rightClickEventOverride = (e: MouseEvent) => { e.preventDefault(); };

    // Disable the ability to right click when the game loads
    onMount(() => {
        window.addEventListener('contextmenu', rightClickEventOverride); 
    });
    // And re-enable it when the game is over
    onCleanup(() => {
        window.removeEventListener('contextmenu', rightClickEventOverride);
    });

    app.setGameState(GameState.BeforeGame);
    props.setGameSettings();

    return <div class="gamescreen">
        <button onClick={() => app.setScreen(Screen.MainMenu)}>Back</button>
        <br />
        Mines remaining: {minesRemaining()}
        <br />
        Cells remaining: {hiddenCellCount()}
        <br />
        Flags: {flagCount()}
        <br />
        <Switch>
            <Match when={app.gameState() === GameState.GameOver }>
                Git gud
            </Match>
            <Match when={app.gameState() === GameState.GameWon }>
                Congratulations, you won!
            </Match>
        </Switch>
        <br />
        <Board {...boardprops} />
    </div>
}