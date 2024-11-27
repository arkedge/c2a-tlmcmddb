# coding: UTF-8
"""
python 3.7以上を要求
"""
import os
import sys
import csv
import re
import pprint

# 環境変数
TLM_DB_PATH = "../TLM_DB/"
TLM_DB_PREFIX = "SAMPLE_TLM_DB_"


def main():
    os.chdir(TLM_DB_PATH)

    csv_files = [
        f for f in os.listdir() if re.match(r"^" + TLM_DB_PREFIX + ".*\.csv$", f)
    ]

    for file in csv_files:
        convert(file)

    print("Completed!")
    sys.exit(0)


def convert(file):
    print("FILE: " + file)
    sheet = load_csv(file)

    # 0 - 4 行は A1 セルを除いてそのまま
    sheet[0][0] = ""

    # 5 - 21 行は固定
    for i in range(5, 22):
        sheet[i] = get_header(i)

    # 22 - 499 は変換
    for i in range(22, 500):
        if sheet[i][4] != "":
            sheet[i][4] = "PACKET"
        sheet[i][16] = sheet[i][19]
        sheet[i][17] = sheet[i][20]
        if sheet[i][7] == get_v3_bitlen_formula():
            sheet[i][7] = get_v2_bitlen_formula()

    # 行を 18 行に
    for i in range(len(sheet)):
        sheet[i] = sheet[i][:18]

    output_csv(file, sheet)


def load_csv(file):
    with open(file, mode="r", encoding="utf-8") as fh:
        reader = csv.reader(fh)
        sheet = [row for row in reader]
    return sheet


def output_csv(file, sheet):
    if len(sheet) != 500:
        print("The number of lines is invalid")
        sys.exit(1)
    with open(file, "w", encoding="utf-8") as fh:
        for line in sheet:
            if len(line) != 18:
                print("The number of columns is invalid")
                sys.exit(1)
            fh.write(",".join(line))
            fh.write("\n")


def get_v2_bitlen_formula():
    return '=IF(OR(EXACT(RC[-5]@@"uint8_t")@@EXACT(RC[-5]@@"int8_t"))@@8@@IF(OR(EXACT(RC[-5]@@"uint16_t")@@EXACT(RC[-5]@@"int16_t"))@@16@@IF(OR(EXACT(RC[-5]@@"uint32_t")@@EXACT(RC[-5]@@"int32_t")@@EXACT(RC[-5]@@"float"))@@32@@IF(EXACT(RC[-5]@@"double")@@64))))'


def get_v3_bitlen_formula():
    return '=IF(OR(EXACT(RC[-5]@@"uint8_t")@@EXACT(RC[-5]@@"int8_t"))@@8@@IF(OR(EXACT(RC[-5]@@"uint16_t")@@EXACT(RC[-5]@@"int16_t"))@@16@@IF(OR(EXACT(RC[-5]@@"uint32_t")@@EXACT(RC[-5]@@"int32_t")@@EXACT(RC[-5]@@"float"))@@32@@IF(EXACT(RC[-5]@@"double")@@64@@IF(EXACT(RC[-5]@@"raw")@@" ")))))'


def get_header(line):
    header = [
        [
            "Comment",
            "TLM Entry",
            "Onboard Software Info.",
            "",
            "Extraction Info.",
            "",
            "",
            "",
            "Conversion Info.",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "Description",
            "Note",
        ],
        [
            "",
            "Name",
            "Var.%%##Type",
            "Variable or Function Name",
            "Ext.##Type",
            "Pos. Desiginator",
            "",
            "",
            "Conv.%%##Type",
            "Poly (Σa_i * x^i)",
            "",
            "",
            "",
            "",
            "",
            "Status",
            "",
            "",
        ],
        [
            "",
            "",
            "",
            "",
            "",
            "Octet%%##Pos.",
            "bit%%##Pos.",
            "bit%%##Len.",
            "",
            "a0",
            "a1",
            "a2",
            "a3",
            "a4",
            "a5",
            "",
            "",
            "",
        ],
        [
            "",
            "PH.VER",
            "uint16_t",
            "",
            "PACKET",
            "0",
            "0",
            "3",
            "NONE",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ],
        [
            "",
            "PH.TYPE",
            "||",
            "",
            "PACKET",
            "=R[-1]C+INT((R[-1]C[1]+R[-1]C[2])/8)",
            "=MOD((R[-1]C+R[-1]C[1])@@8)",
            "1",
            "NONE",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ],
        [
            "",
            "PH.SH_FLAG",
            "||",
            "",
            "PACKET",
            "=R[-1]C+INT((R[-1]C[1]+R[-1]C[2])/8)",
            "=MOD((R[-1]C+R[-1]C[1])@@8)",
            "1",
            "NONE",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ],
        [
            "",
            "PH.APID",
            "||",
            "",
            "PACKET",
            "=R[-1]C+INT((R[-1]C[1]+R[-1]C[2])/8)",
            "=MOD((R[-1]C+R[-1]C[1])@@8)",
            "11",
            "NONE",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ],
        [
            "",
            "PH.SEQ_FLAG",
            "uint16_t",
            "",
            "PACKET",
            "=R[-1]C+INT((R[-1]C[1]+R[-1]C[2])/8)",
            "=MOD((R[-1]C+R[-1]C[1])@@8)",
            "2",
            "NONE",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ],
        [
            "",
            "PH.SEQ_COUNT",
            "||",
            "",
            "PACKET",
            "=R[-1]C+INT((R[-1]C[1]+R[-1]C[2])/8)",
            "=MOD((R[-1]C+R[-1]C[1])@@8)",
            "14",
            "NONE",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ],
        [
            "",
            "PH.PACKET_LEN",
            "uint16_t",
            "",
            "PACKET",
            "=R[-1]C+INT((R[-1]C[1]+R[-1]C[2])/8)",
            "=MOD((R[-1]C+R[-1]C[1])@@8)",
            get_v2_bitlen_formula(),
            "NONE",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ],
        [
            "",
            "SH.VER",
            "uint8_t",
            "",
            "PACKET",
            "=R[-1]C+INT((R[-1]C[1]+R[-1]C[2])/8)",
            "=MOD((R[-1]C+R[-1]C[1])@@8)",
            get_v2_bitlen_formula(),
            "NONE",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ],
        [
            "",
            "SH.TI",
            "uint32_t",
            "",
            "PACKET",
            "=R[-1]C+INT((R[-1]C[1]+R[-1]C[2])/8)",
            "=MOD((R[-1]C+R[-1]C[1])@@8)",
            get_v2_bitlen_formula(),
            "NONE",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ],
        [
            "",
            "SH.TLM_ID",
            "uint8_t",
            "",
            "PACKET",
            "=R[-1]C+INT((R[-1]C[1]+R[-1]C[2])/8)",
            "=MOD((R[-1]C+R[-1]C[1])@@8)",
            get_v2_bitlen_formula(),
            "HEX",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ],
        [
            "",
            "SH.GLOBAL_TIME",
            "double",
            "",
            "PACKET",
            "=R[-1]C+INT((R[-1]C[1]+R[-1]C[2])/8)",
            "=MOD((R[-1]C+R[-1]C[1])@@8)",
            get_v2_bitlen_formula(),
            "NONE",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ],
        [
            "",
            "SH.ON_BOARD_SUBNET_TIME",
            "uint32_t",
            "",
            "PACKET",
            "=R[-1]C+INT((R[-1]C[1]+R[-1]C[2])/8)",
            "=MOD((R[-1]C+R[-1]C[1])@@8)",
            get_v2_bitlen_formula(),
            "NONE",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ],
        [
            "",
            "SH.DEST_FLAGS",
            "uint8_t",
            "",
            "PACKET",
            "=R[-1]C+INT((R[-1]C[1]+R[-1]C[2])/8)",
            "=MOD((R[-1]C+R[-1]C[1])@@8)",
            get_v2_bitlen_formula(),
            "HEX",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ],
        [
            "",
            "SH.DEST_INFO",
            "uint8_t",
            "",
            "PACKET",
            "=R[-1]C+INT((R[-1]C[1]+R[-1]C[2])/8)",
            "=MOD((R[-1]C+R[-1]C[1])@@8)",
            get_v2_bitlen_formula(),
            "NONE",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ],
    ]

    return header[line - 5]


if __name__ == "__main__":
    main()
