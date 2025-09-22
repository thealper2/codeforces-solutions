import os

def create_folders(base_dir='.', max_question=4000):
    for lower_bound in range(0, max_question, 100):
        upper_bound = lower_bound + 99
        folder_name = f"{lower_bound:04d}-{upper_bound:04d}"
        folder_path = os.path.join(base_dir, folder_name)
        os.makedirs(folder_path, exist_ok=True)
        print(f"Created folder: {folder_name}")

if __name__ == "__main__":
    print("Creating folders...")
    create_folders()
