import distutils.spawn

def check(executable: str) -> bool:
    return distutils.spawn.find_executable(executable) is not None