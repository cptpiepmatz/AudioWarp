import {prompt} from "inquirer";
import {AntiAFKMeasure, antiAFKMeasures} from "./startAntiAFKMeasure";

/**
 * Function to select which anti afk measure to use.
 */
export default async function selectAntiAFKMeasure(): Promise<AntiAFKMeasure> {
  const {measure} = await prompt([
    {
      type: "list",
      name: "measure",
      message: "Select key pressed regularly as anti afk measure",
      choices: antiAFKMeasures
    }
  ]);

  return measure;
}
