#!/usr/bin/env python
# -*- coding: utf-8 -*-
# LoLUpdater for OS X v2.0.3
# Ported by David Knaack
# LoLUpdater for Windows: https://lolupdater.com
# License: GPLv3 http://www.gnu.org/licenses/gpl-3.0.html

from __future__ import print_function, unicode_literals

import hashlib
import os
import plistlib
import shutil
import sys
import tarfile
import tempfile
from os.path import join
from subprocess import call

try:
    from urllib.request import urlopen
except ImportError:
    from urllib2 import urlopen

__version__ = '2.0.2'


def main():
    args = sys.argv[1:]
    lol_path = '/Applications/League of Legends.app'
    if len(args) == 0:
        updater = LoLUpdater(lol_path)
        updater.install()
    elif len(args) == 1:
        if args[0] == 'install':
            updater = LoLUpdater(lol_path)
            updater.install()
        elif args[0] == 'uninstall':
            updater = LoLUpdater(lol_path)
            updater.uninstall()
        else:
            print('Unknown command ' + args[0] + '!', file=sys.stderr)
            exit(os.EX_USAGE)
    elif len(args) == 2:
        updater = LoLUpdater(args[1])
        if args[0] == 'install':
            updater.install()
        elif args[0] == 'uninstall':
            updater.uninstall()
    else:
        print('Too many Arguments!', file=sys.stderr)
        exit(os.EX_USAGE)


class LoLUpdater:
    def __init__(self, lol_path):
        print('LoLUpdater {0}'.format(__version__))

        try:
            verify_lol_path(lol_path)
        except NameError:
            # sorry python 2
            pass

        self.lol_path = lol_path
        self.backups = join(self.lol_path, 'Backups')
        self.update_paths = get_paths(lol_path)

        print('Report errors, feature requests or any issues at https://github.com/LoLUpdater/LoLUpdater-OSX/issues.')

    def install(self):
        temp_dir = tempfile.mkdtemp('LolUpdater')
        self.backup()
        air_framework = get_air(temp_dir)
        patch_all('Adobe Air', air_framework, {self.update_paths['air']})

        bugsplat_framework = get_bugsplat(temp_dir)
        patch_all('BugSplat', bugsplat_framework,
                  {self.update_paths['game_client'], self.update_paths['play'], self.update_paths['solution'],
                   self.update_paths['user_kernel']})

        cg_framework = get_cg(temp_dir)
        patch_all('Cg', cg_framework,
                  {self.update_paths['game_client'], self.update_paths['solution']})

        cleanup(temp_dir)
        print('Finished! LoL is now updated. You will need to rerun the script as soon as the client gets updated.')

    def uninstall(self):
        backups = self.backups
        air_framework = join(backups, 'Adobe Air.framework')
        patch_all('Adobe Air', air_framework, [self.update_paths['air']])

        bugsplat_framework = join(backups, 'BugSplat.framework')
        patch_all('BugSplat', bugsplat_framework,
                  {self.update_paths['game_client'], self.update_paths['play'], self.update_paths['solution'],
                   self.update_paths['user_kernel']})

        cg_framework = join(backups, 'Cg.framework')
        patch_all('Cg', cg_framework,
                  {self.update_paths['game_client'], self.update_paths['solution']})

        print('Finished! LoL is now restored. You will need to rerun the script as soon as the client gets updated.')

    def backup(self):
        print('Creating Backups…')
        backups = self.backups
        backup_framework('Adobe Air', backups, self.update_paths['air'])
        backup_framework('BugSplat', backups, self.update_paths['solution'])
        backup_framework('Cg', backups, self.update_paths['game_client'])


def get_air(temp_dir):
    print('Downloading Adobe Air (this might take a while)…')
    download_url = 'https://airdownload.adobe.com/air/mac/download/19.0/AdobeAIR.dmg'
    download_destination = join(temp_dir, 'air.dmg')
    download(download_url, download_destination)

    print('Mounting Adobe Air…')
    mountpoint = mount(download_destination)

    air_framework = join(temp_dir, 'Adobe Air.framework')
    shutil.copytree(join(mountpoint, 'Adobe Air Installer.app/Contents/Frameworks/Adobe Air.framework'),
                    air_framework)

    print('Unmounting Adobe Air…')
    unmount(mountpoint)

    return air_framework


def get_bugsplat(temp_dir):
    print('Downloading BugSplat and verifying checksum…')
    download_url = 'https://www.bugsplatsoftware.com/files/MyCocoaCrasher.dmg'
    download_destination = join(temp_dir, 'bugsplat.dmg')
    download_hash = '09f9d5d54a90cb93b01844f31f8d7fcb3c216d25b4fbdff5d7058b49b4671c7c'
    download(download_url, download_destination, download_hash)

    print('Mounting BugSplat…')
    mountpoint = mount(download_destination)

    bugsplat_framework = join(temp_dir, 'BugSplat.framework')
    shutil.copytree(join(mountpoint, 'MyCocoaCrasher/BugSplat.framework'),
                    bugsplat_framework)

    print('Unmounting BugSplat…')
    unmount(mountpoint)

    return bugsplat_framework


def get_cg(temp_dir):
    print('Downloading Cg and verifying checksum…')
    download_url = 'http://developer.download.nvidia.com/cg/Cg_3.1/Cg-3.1_April2012.dmg'
    download_destination = join(temp_dir, 'cg.dmg')
    download_hash = '85c7a0de82252b703191fee5fe7b29f60d357924dc7b8ca59c2badeac7af407d'
    download(download_url, download_destination, download_hash)

    print('Mounting Cg…')
    mountpoint = mount(download_destination)

    shutil.copy(join(mountpoint, 'Cg-3.1.0013.app/Contents/Resources/Installer Items/NVIDIA_Cg.tgz'), temp_dir)

    cg_framework = join(temp_dir, 'Cg.framework')
    with tarfile.open(join(temp_dir, 'NVIDIA_Cg.tgz'), 'r:gz') as tar:
        extract_cg(tar, cg_framework)

    print('Unmounting Cg…')
    unmount(mountpoint)

    return cg_framework


def cleanup(item):
    print('Cleaning up…')
    shutil.rmtree(item)


def mount(filename):
    mountpoint = tempfile.mkdtemp('LolUpdaterMount')
    call(['hdiutil', 'attach', '-nobrowse', '-quiet', '-mountpoint', mountpoint, filename])
    return mountpoint


def unmount(mountpoint):
    call(['hdiutil', 'detach', '-quiet', mountpoint])
    shutil.rmtree(mountpoint)


def find_path(lol_path, path_head, path_tail):
    _path_head = join(lol_path, path_head)
    for i in os.listdir(_path_head):
        potential_path = join(_path_head, i, path_tail)
        if os.path.isdir(potential_path):
            return potential_path


def get_paths(lol_path):
    return {
        'air':         find_path(lol_path,
                                 'Contents/LoL/RADS/projects/lol_air_client/releases',
                                 'deploy/Frameworks'),
        'game_client': find_path(lol_path,
                                 'Contents/LoL/RADS/solutions/lol_game_client_sln/releases',
                                 'deploy/LeagueOfLegends.app/Contents/Frameworks'),
        'play':        join(lol_path, 'Contents/LoL/RADS/system/UserKernel.app/Contents/Frameworks'),
        'solution':    find_path(lol_path,
                                 'Contents/LoL/RADS/projects/lol_game_client/releases',
                                 'deploy/LeagueOfLegends.app/Contents/Frameworks'),
        'user_kernel': join(lol_path, 'Contents/LoL/Play League of Legends.app/Contents/Frameworks')

    }


def patch_all(name, source, destinations):
    print('Patching ' + name + '…')
    for destination in destinations:
        patch_tree(source, join(destination, name + '.framework'))


def backup_framework(name, backups, source):
    framework_name = name + '.framework'
    destination = join(backups, framework_name)
    if os.path.isdir(destination):
        print('Backup for ' + name + ' already exists. Skipping!')
    else:
        print('Backing up ' + name + '…')
        shutil.copytree(join(source, framework_name), destination)


def patch_tree(source, destination):
    names = os.listdir(source)

    for name in names:
        source_name = join(source, name)
        destination_name = join(destination, name)

        if os.path.isdir(source_name):
            if not os.path.isdir(destination_name):
                os.mkdir(destination_name)
            patch_tree(source_name, destination_name)
        elif os.path.isfile(source_name) and not os.path.islink(destination_name):
            if os.path.isdir(destination_name):
                shutil.rmtree(destination_name)
            copyfile(source_name, destination_name)


def download(url, destination, file_hash=None):
    request = urlopen(url)
    with open(destination, 'wb') as file_pointer:
        copy_download(request, file_pointer, file_hash)


def copy_download(fp_src, fp_dst, file_hash=None):
    global m
    if file_hash is not None:
        m = hashlib.sha256()
    while 1:
        buffer = fp_src.read(16 * 1024)
        if not buffer:
            break
        fp_dst.write(buffer)
        if file_hash is not None:
            m.update(buffer)
    if file_hash is not None and m.hexdigest() != file_hash:
        raise Exception('File does not have correct checksum!')


def extract_cg(members, path):
    print('Extracting Cg…')
    os.mkdir(path)
    for tarinfo in members:
        if tarinfo.name.startswith('Library/Frameworks/Cg.framework/'):
            name = tarinfo.name.replace('Library/Frameworks/Cg.framework/', '', 1)
            if tarinfo.isdir():
                os.mkdir(join(path, name))
            elif tarinfo.isfile():
                members.extract(tarinfo, join(path, name))


def copyfile(src, dst):
    with open(src, 'rb') as fp_src:
        with open(dst, 'wb+') as fp_dst:
            shutil.copyfileobj(fp_src, fp_dst)


def verify_lol_path(lol_path):
    try:
        with open(join(lol_path, 'Contents/Info.plist'), 'rb') as fp:
            plist = plistlib.load(fp)
            if plist['CFBundleIdentifier'] != 'com.riotgames.MacContainer':
                print('Could not detect valid LoL installation at "' + lol_path + '"!', file=sys.stderr)
                exit(os.EX_DATAERR)
    except FileNotFoundError:
        print('Could not detect valid LoL installation at "' + lol_path + '"!', file=sys.stderr)
        exit(os.EX_DATAERR)


if __name__ == "__main__":
    main()
