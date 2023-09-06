import bcrypt

def validate_pwd(pwd):
    hash = "VABB7yO9xm7xWXROeASsmsgnY0o0sDMJev7zFHhwQS8mvM8V5xQQp"

    passwords = [pwd]
    passwords.append(pwd.upper())
    passwords.append(pwd.capitalize())

    for password in passwords:
        for rounds in range (11, 31):
            full_hash = (
                b"$"
                + b"2a"
                + b"$"
                + ("%2.2u" % rounds).encode("ascii")
                + b"$"
                + hash.encode("ascii")
            )
            b = password.encode('ascii')

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
