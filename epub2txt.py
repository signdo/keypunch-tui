#!/usr/bin/python3
import shutil, os, sys, zipfile, tempfile
from lxml import etree

if len(sys.argv) != 3:
    print(f"Usage: {sys.argv[0]} <input.epub> <output.txt>")
    exit(1)

inputFilePath=sys.argv[1]
outputFilePath=sys.argv[2]

print(f"Input: {inputFilePath}")
print(f"Output: {outputFilePath}")

with tempfile.TemporaryDirectory() as tmpDir:
    print(f"Extracting input to temp directory '{tmpDir}'.")
    with zipfile.ZipFile(inputFilePath, 'r') as zip_ref:
        zip_ref.extractall(tmpDir)

    with open(outputFilePath, "w") as outFile:
        print(f"Parsing 'container.xml' file.")
        containerFilePath=f"{tmpDir}/META-INF/container.xml"
        tree = etree.parse(containerFilePath)
        for rootFilePath in tree.xpath( "//*[local-name()='container']"
                                        "/*[local-name()='rootfiles']"
                                        "/*[local-name()='rootfile']"
                                        "/@full-path"):
            print(f"Parsing '{rootFilePath}' file.")
            contentFilePath = f"{tmpDir}/{rootFilePath}"
            contentFileDirPath = os.path.dirname(contentFilePath)

            tree = etree.parse(contentFilePath)
            for idref in tree.xpath("//*[local-name()='package']"
                                    "/*[local-name()='spine']"
                                    "/*[local-name()='itemref']"
                                    "/@idref"):
                for href in tree.xpath( f"//*[local-name()='package']"
                                        f"/*[local-name()='manifest']"
                                        f"/*[local-name()='item'][@id='{idref}']"
                                        f"/@href"):
                    outFile.write("\n")
                    xhtmlFilePath = f"{contentFileDirPath}/{href}"
                    subtree = etree.parse(xhtmlFilePath, etree.HTMLParser())
                    for ptag in subtree.xpath("//html/body/*"):
                        for text in ptag.itertext():
                            outFile.write(f"{text}")
                        outFile.write("\n")

print(f"Text written to '{outputFilePath}'.")
