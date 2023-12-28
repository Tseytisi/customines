import "./board.css";
import "./tempboardui.css";
import { getLogic } from "./boardlogic";
import { Show } from "solid-js";

export default function Cell(props: any) {
    const logic: any = getLogic();

    return <div classList={{ 
        cell: true, 
        hidden: logic.getState(props.x, props.y) === "Hidden",
        flagged: logic.getState(props.x, props.y) === "Flagged",
        questioned: logic.getState(props.x, props.y) === "Questioned",
        showValue: logic.getState(props.x, props.y) === "ShowValue",
        showBlank: logic.getState(props.x, props.y) === "ShowValue" && logic.getValue(props.x, props.y) === 0,
        showValue1: logic.getState(props.x, props.y) === "ShowValue" && logic.getValue(props.x, props.y) === 1,
        showValue2: logic.getState(props.x, props.y) === "ShowValue" && logic.getValue(props.x, props.y) === 2,
        showValue3: logic.getState(props.x, props.y) === "ShowValue" && logic.getValue(props.x, props.y) === 3,
        showValue4: logic.getState(props.x, props.y) === "ShowValue" && logic.getValue(props.x, props.y) === 4,
        showValue5: logic.getState(props.x, props.y) === "ShowValue" && logic.getValue(props.x, props.y) === 5,
        showValue6: logic.getState(props.x, props.y) === "ShowValue" && logic.getValue(props.x, props.y) === 6,
        showValue7: logic.getState(props.x, props.y) === "ShowValue" && logic.getValue(props.x, props.y) === 7,
        showValue8: logic.getState(props.x, props.y) === "ShowValue" && logic.getValue(props.x, props.y) === 8,
        showMine: logic.getState(props.x, props.y) === "ShowValue" && logic.getValue(props.x, props.y) === 9,
        showInvalidMine: logic.getState(props.x, props.y) === "ShowInvalidMine",
        showMineExploded: logic.getState(props.x, props.y) === "ShowMineExploded"
    }} 
        onClick={() => logic.poke(props.x, props.y)}
        onContextMenu={() => logic.mark(props.x, props.y)}
    >
        <Show when={
            logic.getState(props.x, props.y) === "ShowValue" &&
            logic.getValue(props.x, props.y) !== 0 &&
            logic.getValue(props.x, props.y) !== 9
        }>
            <p class="cellValue">
                {logic.getValue(props.x, props.y)}
            </p>
        </Show>
    </div>
}