/** Possible anti afk measures. */

import robotjs from "robotjs";

/** Possible anti afk measure keys. */
export const antiAFKMeasures = ["none", "alt", "control", "esc"];

/** Type representing the anti afk measure key. */
export type AntiAFKMeasure = typeof antiAFKMeasures[number];

/**
 * Starts an interval for the anti afk measures.
 * If the measure select is not "none" it will press the key every minute.
 * @param antiAFKMeasure Key to press regularly
 */
export default function startAntiAFKMeasure(antiAFKMeasure: AntiAFKMeasure) {
  if (antiAFKMeasure === antiAFKMeasures[0]) return;
  return setInterval(() => robotjs.keyTap(antiAFKMeasure), 60 * 1000);
}
