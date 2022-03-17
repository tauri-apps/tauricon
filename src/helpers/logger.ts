// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

import chalk from 'chalk';
import ms from 'ms';

let prevTime: number;

const logger = (banner: string, color: chalk.Chalk = chalk.green) => {
  return (msg?: string): void => {
    const curr = +new Date();
    const diff = curr - (prevTime || curr);

    prevTime = curr;

    if (msg) {
      console.log(
        // TODO: proper typings for color and banner
        // eslint-disable-next-line @typescript-eslint/restrict-template-expressions, @typescript-eslint/no-unsafe-call
        ` ${color(String(banner))} ${msg} ${chalk.green(`+${ms(diff)}`)}`,
      );
    } else {
      console.log();
    }
  };
};

export default logger;
