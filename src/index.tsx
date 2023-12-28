/* @refresh reload */
import { render } from "solid-js/web";

import "./styles.css";
import {Customines, CustominesProvider} from "./customines.tsx";

render(() =>
    <CustominesProvider>
        <Customines />
    </CustominesProvider>
    , document.getElementById("root") as HTMLElement);
