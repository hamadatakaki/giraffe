from os import chmod, makedirs, stat


def create_directories(paths: list, mode: dict):
    for p in paths:
        makedirs(p)
        chmod(p, mode[p])


def create_file(path: str, mode: int, body: bytearray):
    with open(path, 'wb') as wb:
        wb.write(body)

    chmod(path, mode)
