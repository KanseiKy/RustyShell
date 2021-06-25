import colorama

RED = colorama.Fore.RED
GREEN = colorama.Fore.GREEN
RESET = colorama.Style.RESET_ALL

colorama.init()

def error(msg: str) -> None:
    print(f'({RED}Error{RESET}): {msg}')
    return

def info(msg: str) -> None:
    print(f'({GREEN}Info{RESET}): {msg}')
    return