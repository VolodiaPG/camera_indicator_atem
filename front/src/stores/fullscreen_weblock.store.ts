import NoSleep from "nosleep.js";
import * as _screenfull from "screenfull";
import type { Screenfull } from "screenfull";
import { readable } from "svelte/store";
const screenfull = _screenfull as Screenfull;

const noSleep = new NoSleep();

export let wakelock_fullscreen;

export const fullscreen = readable(false, (set) => {
    wakelock_fullscreen = () => {
        if (!noSleep.isEnabled) {
            noSleep.enable();
        } else {
            noSleep.disable();
        }

        if (screenfull.isEnabled) {
            screenfull.toggle();
            screenfull.onchange(() => {
                set(screenfull.isFullscreen);
            });
        }
    }
});