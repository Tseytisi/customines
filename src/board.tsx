import { For } from "solid-js";
import "./board.css";
import Cell from "./cell.tsx";
import { BoardLogicProvider } from "./boardlogic.tsx";

export default function Board(props: any) {
    const coords_x = [...Array(props.boardWidth())];
    const coords_y = [...Array(props.boardHeight())];

    return <div id="gameboard">
        <BoardLogicProvider {...props}>
            <For each={coords_y}>{ (_, y) => (
                <div class="boardrow">
                    <For each={coords_x}>{(_, x) => (
                        <Cell x={x()} y={y()} />
                        )}
                    </For>
                </div>
                )}
            </For>
        </BoardLogicProvider>
    </div>
}
