import os
import zlib

PROJECT_PATH = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
REPO_PATH = os.path.join(PROJECT_PATH, 'repo')
paths = {
    'blob': os.path.join(REPO_PATH, 'blobobj'),
    'commit': os.path.join(REPO_PATH, 'commitobj'),
    'index': os.path.join(REPO_PATH, 'index'),
    'tree': os.path.join(REPO_PATH, 'treeobj')
}


class DataStruct:
    def __init__(self, b: bytearray):
        sep = b.index(b'\x00')
        header, body = b[:sep], b[sep+1:]
        self.body = body.decode()
        t, l = header.decode().split(' ')
        self.type = t
        self.length = int(l)

    def __str__(self):
        return f'{self.type[:4]}: {self.length}'

def get_bytes_struct_from_path(path: str):
    with open(path, 'rb') as rb:
        b = rb.read()

    dc = zlib.decompress(b)
    return DataStruct(dc)

def cat_file(path: str):
    bd = get_bytes_struct_from_path(path)
    print(bd.body, end='')


if __name__ == '__main__':
    testblobpath = '/Users/hamada/develop/giraffe/.git/objects/e7/a11a969c037e00a796aafeff6258501ec15e9a'
    testcommitpath = '/Users/hamada/develop/giraffe/.git/objects/cd/f7e6cbf6a5b18c2c189f9c2b934cbc6632e647'
    cat_file(testblobpath)
    print()
    cat_file(testcommitpath)
