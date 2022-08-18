import inquirer from "inquirer";

import {antiAFKMeasures, AntiAFKMeasure} from "./startAntiAFKMeasure.js";

/**
 * Function to select which anti afk measure to use.
 */
export default async function selectAntiAFKMeasure(): Promise<AntiAFKMeasure> {
  const {measure} = await inquirer.prompt([
    {
      type: "list",
      name: "measure",
      message: "Select key pressed regularly as anti afk measure",
      choices: antiAFKMeasures
    }
  ]);

  return measure;
}
