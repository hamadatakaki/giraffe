import os

from lib.structure import get_bytes_struct_from_path, analyze_index

PROJECT_PATH = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
REPO_PATH = os.path.join(PROJECT_PATH, 'repo')
paths = {
    'blob': os.path.join(REPO_PATH, 'blobobj'),
    'commit': os.path.join(REPO_PATH, 'commitobj'),
    'index': os.path.join(REPO_PATH, 'index'),
    'tree': os.path.join(REPO_PATH, 'treeobj')
}

def cat_file(path: str):
    bd = get_bytes_struct_from_path(path)
    print(bd.body, end='')


def ls_files(path: str):
    index = analyze_index(path)
    for e in index.entries:
        print(f'{e.name}')


def ls_files_stage(path):
    index = analyze_index(path)
    for e in index.entries:
        print(f'{e.mode} {e.sha1}\t{e.name}')


if __name__ == '__main__':
    ls_files_stage(paths['index'])
