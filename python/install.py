import colorama
import sys
import os

colorama.init

from .log import info

def install(pkg: str) -> None:
    if pkg not in sys.modules:
        info(f'Installing {pkg}...')
        os.system(f'python -m pip install {pkg}')
    
    return