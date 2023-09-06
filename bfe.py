import bcrypt

def validate_pwd(pwd):
    hash = "VABB7yO9xm7xWXROeASsmsgnY0o0sDMJev7zFHhwQS8mvM8V5xQQp"

    passwords = [pwd]
    passwords.append(pwd.upper())
    passwords.append(pwd.capitalize())

    for password in passwords:
        full_hash = "$2a$31$" + hash
        full_hash = full_hash.encode('utf-8')
        b = password.encode('utf-8')

        if bcrypt.checkpw(b, full_hash):
            print("Match:")
            print(f"pwd: {password}")
        print()

    return False

if __name__ == "__main__":


    pwds = [
        "god",
    ]
    for pwd in pwds:
        is_match = validate_pwd(pwd)
        print(is_match)
