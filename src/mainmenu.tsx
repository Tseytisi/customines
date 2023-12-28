import { getContext, MAX_BOARD_HEIGHT, MAX_BOARD_WIDTH, Screen } from "./customines.tsx"
import { Show } from "solid-js";

export default function MainMenu(props: any) {
    const settings = getContext();

    function increaseBoardWidth() { 
        let old_value = props.boardWidth();
        if (old_value < MAX_BOARD_WIDTH) {
            props.setBoardWidth((c: number) => c + 1);
            // If the previous mine-count was the max, we increase it to the new max
            let old_max = old_value * props.boardHeight() - 9;
            if (props.mineCount() == old_max) props.setMineCount((old_value + 1) * props.boardHeight() - 9);
        }
    }
    function increaseBoardHeight() {
        let old_value = props.boardHeight();
        if (old_value < MAX_BOARD_HEIGHT) {
            props.setBoardHeight((c: number) => c + 1);
            // If the previous mine-count was the max, we increase it to the new max
            let old_max = old_value * props.boardWidth() - 9;
            if (props.mineCount() == old_max) props.setMineCount((old_value + 1) * props.boardWidth() - 9);
        }
    }

    function decreaseBoardWidth() {
        let old_value = props.boardWidth(); 
        if (old_value > 4) {
            props.setBoardWidth((c: number) => c - 1);
            // If the previous mine-count is greater than our new max, set it to the new max
            let new_max = (old_value - 1) * props.boardHeight() - 9;
            if (props.mineCount() > new_max) {
                props.setMineCount(new_max);
            }
        }
    }
    function decreaseBoardHeight() {
        let old_value = props.boardHeight();
        if (old_value > 4) {
            props.setBoardHeight((c: number) => c - 1);
            // If the previous mine-count is greater than our new max, set it to the new max
            let new_max = (old_value - 1) * props.boardWidth() - 9;
            if (props.mineCount() > new_max) {
                props.setMineCount(new_max);
            }
        }
    }
    function decreaseMineCount() { if (props.mineCount() > 1) props.setMineCount((c: number) => c - 1) }
    function increaseMineCount() { 
        if (props.mineCount() < (props.boardHeight() * props.boardWidth() - 9)) 
            props.setMineCount((c: number) => c + 1) 
    }

    return <div id="mainmenu">
        <button onClick={() => settings.setScreen(Screen.Game)}>Play</button>
        <br />
        <div style="display: inline;">
            Board width: 
            <button onClick={decreaseBoardWidth}>-</button>
            {props.boardWidth()}
            <button onClick={increaseBoardWidth}>+</button>
        </div>
        <br />
        <div style="display: inline;">
            Board height:
            <button onClick={decreaseBoardHeight}>-</button>
            {props.boardHeight()}
            <button onClick={increaseBoardHeight}>+</button>
        </div>
        <br />
        <div style="display: inline;">
            # mines:
            <button onClick={decreaseMineCount}>-</button>
            {props.mineCount()}
            <button onClick={increaseMineCount}>+</button>
        </div>
        <br />
        <div style="display: inline;">
            Enable questions:&nbsp;
            <button onClick={() => props.setQuestionsEnabled((c: boolean) => !c)}>
                <Show when={props.questionsEnabled()} fallback={"No"}>
                    Yes
                </Show>
            </button>
        </div>
    </div>
}