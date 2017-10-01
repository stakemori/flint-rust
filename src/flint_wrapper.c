#include "flint.h"
#include "fmpq.h"
#include "fmpz.h"
#include "fmpz_factor.h"
#include "fmpz_mat.h"
#include "fmpq_mat.h"
#include "fmpz_poly.h"
#include "fmpz_vec.h"

void wrapped_fmpz_clear(fmpz_t f) { fmpz_clear(f); }

void wrapped_fmpz_init(fmpz_t f) { fmpz_init(f); }

void wrapped_fmpz_init_set(fmpz_t f, const fmpz_t g) { fmpz_init_set(f, g); }

void wrapped_fmpz_init_set_si(fmpz_t f, slong g) { fmpz_init_set_si(f, g); }

void wrapped_fmpz_init_set_ui(fmpz_t f, ulong g) { fmpz_init_set_ui(f, g); }

void wrapped_fmpz_set_si(fmpz_t f, slong val) { fmpz_set_si(f, val); }

void wrapped_fmpz_set_ui(fmpz_t f, ulong val) { fmpz_set_ui(f, val); }

void wrapped_fmpz_neg_ui(fmpz_t f, ulong val) { fmpz_neg_ui(f, val); }

void wrapped_fmpz_set_uiui(fmpz_t f, mp_limb_t hi, mp_limb_t lo)
{
  fmpz_set_uiui(f, hi, lo);
}

void wrapped_fmpz_neg_uiui(fmpz_t f, mp_limb_t hi, mp_limb_t lo)
{
  fmpz_neg_uiui(f, hi, lo);
}

void wrapped_fmpz_zero(fmpz_t f) { fmpz_zero(f); }

void wrapped_fmpz_one(fmpz_t f) { fmpz_one(f); }

int wrapped_fmpz_is_zero(const fmpz_t f) { return fmpz_is_zero(f); }

int wrapped_fmpz_is_one(const fmpz_t f) { return fmpz_is_one(f); }

void wrapped_fmpz_swap(fmpz_t f, fmpz_t g) { fmpz_swap(f, g); }

int wrapped_fmpz_is_even(const fmpz_t f) { return fmpz_is_even(f); }

int wrapped_fmpz_is_odd(const fmpz_t f) { return fmpz_is_odd(f); }

void wrapped_fmpz_neg(fmpz_t f1, const fmpz_t f2) { fmpz_neg(f1, f2); }

void wrapped_fmpz_negmod(fmpz_t r, const fmpz_t a, const fmpz_t mod)
{
  fmpz_negmod(r, a, mod);
}

void wrapped_fmpz_mul2_uiui(fmpz_t f, const fmpz_t g, ulong h1, ulong h2)
{
  fmpz_mul2_uiui(f, g, h1, h2);
}

void wrapped_fmpz_divexact2_uiui(fmpz_t f, const fmpz_t g, ulong h1, ulong h2)
{
  fmpz_divexact2_uiui(f, g, h1, h2);
}

void wrapped_flint_randinit(flint_rand_t state) { flint_randinit(state); }

void wrapped_flint_randclear(flint_rand_t state) { flint_randclear(state); }

void wrapped_fmpz_set_ui_smod(fmpz_t f, mp_limb_t x, mp_limb_t m)
{
  fmpz_set_ui_smod(f, x, m);
}

int wrapped_fmpz_is_pm1(const fmpz_t f) { return fmpz_is_pm1(f); }

void wrapped_fmpq_clear(fmpq_t x) { fmpq_clear(x); }
void wrapped_fmpq_init(fmpq_t x) { fmpq_init(x); }
void wrapped_fmpq_zero(fmpq_t x) { fmpq_zero(x); }
void wrapped_fmpq_one(fmpq_t x) { fmpq_one(x); }
int wrapped_fmpq_equal(fmpq_t x, fmpq_t y) { return fmpq_equal(x, y); }
int wrapped_fmpq_sgn(fmpq_t x) { return fmpq_sgn(x); }
int wrapped_fmpq_is_zero(fmpq_t x) { return fmpq_is_zero(x); }
int wrapped_fmpq_is_one(fmpq_t x) { return fmpq_is_one(x); }
void wrapped_fmpq_set(fmpq_t x, fmpq_t y) { fmpq_set(x, y); }
void wrapped_fmpq_neg(fmpq_t x, fmpq_t y) { fmpq_neg(x, y); }
void wrapped_fmpq_abs(fmpq_t x, fmpq_t y) { fmpq_abs(x, y); }

void fmpq_mul_si(fmpq_t res, fmpq_t x, long y)
{
  fmpz_mul_si(fmpq_numref(res), fmpq_numref(x), y);
  fmpz_set(fmpq_denref(res), fmpq_denref(x));
  fmpq_canonicalise(res);
}

void fmpq_div_si(fmpq_t res, fmpq_t x, long y)
{
  fmpz_mul_si(fmpq_denref(res), fmpq_denref(x), y);
  fmpz_set(fmpq_numref(res), fmpq_numref(x));
  fmpq_canonicalise(res);
}
