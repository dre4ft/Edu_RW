import os  # Imports the os module to interact with the operating system
import zipfile  # Imports the zipfile module to create zip files

class ZipArchiver:
    def __init__(self):
        pass  # Empty constructor, does nothing during class initialization

    def create_zip(self, files, zip_name):
        """
        Creates a zip file from a list of files.

        :param files: List of file paths to include in the zip
        :param zip_name: Name of the zip file to create
        """
        with zipfile.ZipFile(zip_name, 'w') as zipf:  # Opens a zip file in write mode
            for file in files:  # Iterates through each file path in the list `files`
                if os.path.isfile(file):  # Checks if the file path corresponds to an existing file
                    zipf.write(file, os.path.basename(file))  # Adds the file to the zip with its base name
                else:
                    # Prints an error message if the file does not exist or is not a regular file
                    print(f"The file '{file}' does not exist or is not a valid file.")
