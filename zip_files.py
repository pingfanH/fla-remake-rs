import zipfile
import os
import shutil
import sys
def zip_folder(folder_path, output_path):
    with zipfile.ZipFile(output_path, 'w', zipfile.ZIP_DEFLATED) as zip_file:
        for root, dirs, files in os.walk(folder_path):
            for file in files:
                file_path = os.path.join(root, file)
                zip_file.write(file_path, os.path.relpath(file_path, folder_path))
    print("success")
    
args=sys.argv
args=' '.join(args[1:])


print(args)
zip_folder(args,args+".fla")