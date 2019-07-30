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


class IndexStruct:
    def __init__(self, version: int, entry_number: int, enrtries=[]):
        self.version = version
        self.entry_number = entry_number
        self.entries = enrtries

    def add_entry(self, entry):
        self.entries.append(entry)

class IndexEntryStruct:
    def __init__(self, ctime: float, mtime: float, dev: int, inode: int, 
                mode: str, uid: int, guid: int, size: int, sha1: str, name: str):
        self.ctime = ctime
        self.mtime = mtime
        self.dev = dev
        self.inode = inode
        self.mode = mode
        self.uid = uid
        self.guid = guid
        self.size = size
        self.sha1 = sha1
        self.name = name


def get_bytes_struct_from_path(path: str):
    with open(path, 'rb') as rb:
        b = rb.read()

    dc = zlib.decompress(b)
    return DataStruct(dc)


def cat_file(path: str):
    bd = get_bytes_struct_from_path(path)
    print(bd.body, end='')


def is_index_file(b: bytearray):
    return b == b'DIRC'


def analyze_index_header(header: bytearray):
    return int.from_bytes(header[0:4], 'big'), int.from_bytes(header[4:8], 'big')


def convert_unixtime(byte_vec):
    sec_byte = byte_vec[:4]
    nano_byte = byte_vec[4:]
    sec = int.from_bytes(sec_byte, 'big')
    nano = int.from_bytes(nano_byte, 'big')
    return sec + nano * 10 ** -len(nano_byte)


def analyze_index(path: str):
    with open(path, 'rb') as rb:
        b = rb.read()

    assert is_index_file(b[:4])

    header, body = b[4:12], b[12:]
    version, entry_number = analyze_index_header(header)
    index_struct = IndexStruct(version, entry_number)

    i = 0
    c = 0
    while(True):
        c += 1
        ctime = convert_unixtime(body[i:i+8])
        i += 8
        mtime = convert_unixtime(body[i:i+8])
        i += 8
        dev = int.from_bytes(body[i:i+4], 'big')
        i += 4
        inode = int.from_bytes(body[i:i+4], 'big')
        i += 4
        mode = format(int.from_bytes(body[i:i+4], 'big'), 'o')
        i += 4
        uid = int.from_bytes(body[i:i+4], 'big')
        i += 4
        guid = int.from_bytes(body[i:i+4], 'big')
        i += 4
        size = int.from_bytes(body[i:i+4], 'big')
        i += 4
        sha1 = body[i:i+20].hex()
        i += 20
        name_len = int.from_bytes(body[i:i+2], 'big')
        i += 2
        name = body[i:i+name_len].decode()
        i += name_len
        padding_size = 8 - (6+name_len)%8
        i += padding_size

        new_entry = IndexEntryStruct(ctime, mtime, dev, inode, mode, uid, guid, size, sha1, name)
        index_struct.add_entry(new_entry)

        if c == entry_number:
            break

    return index_struct


def ls_files(index):
    for e in index.entries:
        print(f'{e.name}')


def ls_files_stage(index):
    for e in index.entries:
        print(f'{e.mode} {e.sha1}\t{e.name}')


if __name__ == '__main__':
    index = analyze_index(paths['index'])
    ls_files(index)
