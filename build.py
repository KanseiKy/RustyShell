#!/usr/bin/env python

import sys
import colorama
import os

colorama.init()

from python.log import error, info
from python.install import install
from python.check import check

def main() -> None:
    install('colorama')

    for i in ['rustc', 'cargo']:
        if check(i) == False:
            error(f'You are required to install {i} and set the PATH correctly')
            os._exit(1)

    info('Building...')
    os.system('cargo build -q')
    info('Done!')
    os._exit(1)

if __name__ == '__main__':
    if sys.version_info[0] < 3:
        error('Python 3.x is required')
        error(f'Installed: {sys.version_info}')
        os._exit(1)
    main()