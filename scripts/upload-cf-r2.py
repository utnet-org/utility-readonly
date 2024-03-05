import boto3
import os
import subprocess

def upload_directory(directory, bucket, s3_client):
    for root, dirs, files in os.walk(directory):
        for file in files:
            file_path = os.path.join(root, file)
            s3_key = os.path.relpath(file_path, directory)
            with open(file_path, 'rb') as data:
                s3_client.upload_fileobj(data, bucket, s3_key)

def run_neard_command(command):
    result = subprocess.run(command, capture_output=True, text=True)
    print(result.stdout)

s3 = boto3.client(
    service_name="s3",
    endpoint_url='https://ec9b597fa02615ca6a0e62b7ff35d0cc.r2.cloudflarestorage.com',
    aws_access_key_id='2131355885fd8671f483c8721136972d',
    aws_secret_access_key='05b1dc1f4fb3af390c9f10b905e5eb40e73f7b1aca34651be33fb034aae51e74',
    region_name="auto",  # Must be one of: wnam, enam, weur, eeur, apac, auto
)

#run_neard_command(["/Users/es/utility/target/debug/uncd", "--home=/Users/es/.unc", "database", "make-snapshot", "--destination=/Users/es/snapshot"])
#run_neard_command(["/Users/es/utility/target/debug/uncd", "--home=/Users/es/snapshot", "database", "compact-database"])

upload_directory('/Users/es/snapshot', 'near', s3)