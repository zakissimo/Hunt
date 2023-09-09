import bcrypt

hsh = "VABB7yO9xm7xWXROeASsmsgnY0o0sDMJev7zFHhwQS8mvM8V5xQQp"

def validate_pwd(pwd):

    passwords = [pwd]
    passwords.append(pwd.upper())
    passwords.append(pwd.capitalize())

    for password in passwords:
        for rounds in range (11, 31):
            full_hash = f"$2b${rounds}${hsh}"
            full_hash = full_hash.encode('utf-8')
            b = password.encode('utf-8')

            if bcrypt.checkpw(b, full_hash):
                print("Match:")
                print(f"pwd: {password}")
            print()

    return False

if __name__ == "__main__":

    pwds = [
        "newton",
        "gravity"
    ]

    for pwd in pwds:
        is_match = validate_pwd(pwd)
        print(is_match)
