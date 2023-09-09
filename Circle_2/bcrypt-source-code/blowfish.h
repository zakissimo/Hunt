/*
 * Author     :  Paul Kocher
 * E-mail     :  pck@netcom.com
 * Date       :  1997
 * Description:  C implementation of the Blowfish algorithm.
 */

#include <stdint.h>

#define MAXKEYBYTES 56 /* 448 bits */

typedef struct {
  int32_t P[16 + 2];
  int32_t S[4][256];
} BLOWFISH_CTX;

void Blowfish_Init(BLOWFISH_CTX *ctx, char *key, int keyLen);

void Blowfish_Encrypt(BLOWFISH_CTX *ctx, int32_t *xl, int32_t *xr);

void Blowfish_Decrypt(BLOWFISH_CTX *ctx, int32_t *xl, int32_t *xr);
