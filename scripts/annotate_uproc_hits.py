#!/usr/bin/env python3
"""docstring"""

import argparse
import csv
import os
import sys

# --------------------------------------------------
def get_args():
    """get args"""
    parser = argparse.ArgumentParser(description='Argparse Python script')
    parser.add_argument('-k', '--kegghits', help='KEGG hits',
                        metavar='FILE', type=str, default='')

    parser.add_argument('-p', '--pfamhits', help='PFAM hits',
                        metavar='FILE', type=str, default='')

    parser.add_argument('-e', '--keggdb', help='KEGG annotations',
                        metavar='FILE', type=str,
                        default='kegg_annotation.tab')

    parser.add_argument('-f', '--pfamdb', help='PFAM annotation',
                        metavar='FILE', type=str,
                        default='pfam_annotation.tab')

    parser.add_argument('-o', '--out_dir', help='Output directory',
                        metavar='DIR', type=str, default='')

    return parser.parse_args()

# --------------------------------------------------
def main():
    """main"""
    args = get_args()
    kegg_hits = args.kegghits
    pfam_hits = args.pfamhits
    kegg_db = args.keggdb
    pfam_db = args.pfamdb
    out_dir = args.out_dir

    if not kegg_hits and not pfam_hits:
        print('--kegghits and/or --pfamhits is require')
        sys.exit(1)

    process_kegg(kegg_hits, kegg_db, out_dir)
    process_pfam(pfam_hits, pfam_db, out_dir)

    print('Done.')

# --------------------------------------------------
def process_kegg(kegg_hits, kegg_db, out_dir):
    """Process KEGG file"""
    if not kegg_hits:
        return

    if kegg_hits and not os.path.isfile(kegg_hits):
        print('--kegghits "{}" is not a file'.format(kegg_hits))
        return

    if kegg_hits and not kegg_db:
        print('Must have --keggdb if --kegghits')
        return

    if kegg_db and not os.path.isfile(kegg_db):
        print('--keggdb "{}" is not a file'.format(kegg_db))
        return

    if not out_dir:
        out_dir = os.path.dirname(os.path.abspath(kegg_hits))

    if not os.path.isdir(out_dir):
        os.makedirs(out_dir)

    kegg = {}
    if kegg_db:
        with open(kegg_db) as csvfile:
            reader = csv.DictReader(csvfile, delimiter='\t')
            for rec in reader:
                kegg[rec['kegg_annotation_id']] = rec

    basename = os.path.basename(kegg_hits)
    out_file = os.path.join(out_dir, basename + '.annotated.tab')
    flds = 'kegg_id count name definition pathway module'.split()
    tab = '\t'
    out_fh = open(out_file, 'wt')

    out_fh.write(tab.join(flds) + '\n')
    rec_num = 0
    for line in open(kegg_hits):
        rec_num += 1
        kegg_id, count = line.rstrip().split(',')
        if kegg_id in kegg:
            kegg_rec = kegg[kegg_id]
            out_fh.write(tab.join([kegg_id,
                                   count,
                                   kegg_rec['name'],
                                   kegg_rec['definition'],
                                   kegg_rec['pathway'],
                                   kegg_rec['module']]) + '\n')
        else:
            sys.stderr.write('Cannot find KEGG ID "{}"\n'.format(kegg_id))

    msg = 'Wrote {} KEGG annotation{} to "{}"\n'
    print(msg.format(rec_num, '' if rec_num == 1 else 's', out_file))

# --------------------------------------------------
def process_pfam(pfam_hits, pfam_db, out_dir):
    """Process PFAM file"""
    if not pfam_hits:
        return

    if pfam_hits and not os.path.isfile(pfam_hits):
        print('--pfamhits "{}" is not a file'.format(pfam_hits))
        return

    if pfam_hits and not pfam_db:
        print('Must have --pfamdb if --pfamhits')
        return

    if pfam_db and not os.path.isfile(pfam_db):
        print('--pfamdb "{}" is not a file'.format(pfam_db))
        return

    if not out_dir:
        out_dir = os.path.dirname(os.path.abspath(pfam_hits))

    if not os.path.isdir(out_dir):
        os.makedirs(out_dir)

    pfam = {}
    if pfam_db:
        with open(pfam_db) as csvfile:
            reader = csv.DictReader(csvfile, delimiter='\t')
            for rec in reader:
                pfam[rec['accession']] = rec

    basename = os.path.basename(pfam_hits)
    out_file = os.path.join(out_dir, basename + '.annotated.tab')
    flds = 'pfam_id count identifier name'.split()
    tab = '\t'
    out_fh = open(out_file, 'wt')

    out_fh.write(tab.join(flds) + '\n')
    rec_num = 0
    for line in open(pfam_hits):
        rec_num += 1
        pfam_id, count = line.rstrip().split(',')
        if pfam_id in pfam:
            pfam_rec = pfam[pfam_id]
            out_fh.write(tab.join([pfam_id,
                                   count,
                                   pfam_rec['accession'],
                                   pfam_rec['name']]) + '\n')
        else:
            sys.stderr.write('Cannot find PFAM ID "{}"\n'.format(pfam_id))

    msg = 'Wrote {} PFAM annotation{} to "{}"\n'
    print(msg.format(rec_num, '' if rec_num == 1 else 's', out_file))

# --------------------------------------------------
if __name__ == '__main__':
    main()
