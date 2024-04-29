use solana_program::program_error::ProgramError;
use solana_program::pubkey;
use solana_program::pubkey::Pubkey;

#[derive(Debug, Clone, Copy)]
pub enum SupportedToken {
    USDC,
    USDT,
    Sol,
    Fida,
    MSol,
    Bonk,
    BAT,
    Pyth,
    BSol,
    Inj,
}

const USDC_MINT: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
const USDT_MINT: Pubkey = pubkey!("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB");
const SOL_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
const FIDA_MINT: Pubkey = pubkey!("EchesyfXePKdLtoiZSL8pBe8Myagyy8ZRqsACNCFGnvp");
const MSOL_MINT: Pubkey = pubkey!("mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So");
const BONK_MINT: Pubkey = pubkey!("DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263");
const BAT_MINT: Pubkey = pubkey!("EPeUFDgHRxs9xxEPVaL6kfGQvCon7jmAWKVUHuux1Tpz");
const PYTH_MINT: Pubkey = pubkey!("HZ1JovNiVvGrGNiiYvEozEVgZ58xaU3RKwX8eACQBCt3");
const BSOL_MINT: Pubkey = pubkey!("bSo13r4TkiE4KumL71LsHTPpL2euBYLFx6h9HP3piy1");
const INJ_MINT: Pubkey = pubkey!("6McPRfPV6bY1e9hLxWyG54W9i9Epq75QBvXg2oetBVTB");

impl SupportedToken {
    pub const fn mint(self) -> Pubkey {
        match self {
            SupportedToken::USDC => USDC_MINT,
            SupportedToken::USDT => USDT_MINT,
            SupportedToken::Sol => SOL_MINT,
            SupportedToken::Fida => FIDA_MINT,
            SupportedToken::MSol => MSOL_MINT,
            SupportedToken::Bonk => BONK_MINT,
            SupportedToken::BAT => BAT_MINT,
            SupportedToken::Pyth => PYTH_MINT,
            SupportedToken::BSol => BSOL_MINT,
            SupportedToken::Inj => INJ_MINT,
        }
    }

    pub const fn from_mint(mint: &Pubkey) -> Result<Self, ProgramError> {
        Ok(match *mint {
            USDC_MINT => SupportedToken::USDC,
            USDT_MINT => SupportedToken::USDT,
            SOL_MINT => SupportedToken::Sol,
            FIDA_MINT => SupportedToken::Fida,
            MSOL_MINT => SupportedToken::MSol,
            BONK_MINT => SupportedToken::Bonk,
            BAT_MINT => SupportedToken::BAT,
            PYTH_MINT => SupportedToken::Pyth,
            BSOL_MINT => SupportedToken::BSol,
            INJ_MINT => SupportedToken::Inj,
            _ => return Err(ProgramError::InvalidArgument),
        })
    }

    pub const fn decimals(self) -> u8 {
        match self {
            SupportedToken::Sol
            | SupportedToken::MSol
            | SupportedToken::Inj
            | SupportedToken::BSol => 9,
            SupportedToken::Bonk => 5,
            SupportedToken::BAT => 8,
            SupportedToken::USDC
            | SupportedToken::USDT
            | SupportedToken::Fida
            | SupportedToken::Pyth => 6,
        }
    }

    pub const fn price_feed(self) -> [u8; 32] {
        match self {
            SupportedToken::USDC => [
                234, 160, 32, 198, 28, 196, 121, 113, 40, 19, 70, 28, 225, 83, 137, 74, 150, 166,
                192, 11, 33, 237, 12, 252, 39, 152, 209, 249, 169, 233, 201, 74,
            ],
            SupportedToken::USDT => [
                43, 137, 185, 220, 143, 223, 159, 52, 112, 154, 91, 16, 107, 71, 47, 15, 57, 187,
                108, 169, 206, 4, 176, 253, 127, 46, 151, 22, 136, 226, 229, 59,
            ],
            SupportedToken::Sol => [
                239, 13, 139, 111, 218, 44, 235, 164, 29, 161, 93, 64, 149, 209, 218, 57, 42, 13,
                47, 142, 208, 198, 199, 188, 15, 76, 250, 200, 194, 128, 181, 109,
            ],
            SupportedToken::Fida => [
                200, 6, 87, 183, 246, 243, 234, 194, 114, 24, 208, 157, 90, 78, 84, 228, 123, 37,
                118, 141, 159, 94, 16, 172, 21, 254, 44, 249, 0, 136, 20, 0,
            ],
            SupportedToken::MSol => [
                194, 40, 154, 106, 67, 210, 206, 145, 198, 245, 92, 174, 195, 112, 244, 172, 195,
                138, 46, 212, 119, 245, 136, 19, 51, 76, 109, 3, 116, 159, 242, 164,
            ],
            SupportedToken::Bonk => [
                114, 176, 33, 33, 124, 163, 254, 104, 146, 42, 25, 170, 249, 144, 16, 156, 185,
                216, 78, 154, 208, 4, 180, 210, 2, 90, 214, 245, 41, 49, 68, 25,
            ],
            SupportedToken::BAT => [
                142, 134, 15, 183, 78, 96, 229, 115, 107, 69, 93, 130, 246, 11, 55, 40, 4, 156, 52,
                142, 148, 150, 26, 221, 95, 150, 27, 2, 253, 238, 37, 53,
            ],
            SupportedToken::Pyth => [
                11, 191, 40, 233, 168, 65, 161, 204, 120, 143, 106, 54, 27, 23, 202, 7, 45, 14,
                163, 9, 138, 30, 93, 241, 195, 146, 45, 6, 113, 149, 121, 255,
            ],
            SupportedToken::BSol => [
                137, 135, 83, 121, 231, 15, 143, 186, 220, 23, 174, 243, 21, 173, 243, 168, 213,
                209, 96, 184, 17, 67, 85, 55, 224, 60, 151, 232, 170, 201, 125, 156,
            ],
            SupportedToken::Inj => [
                122, 91, 193, 210, 181, 106, 208, 41, 4, 140, 214, 57, 100, 179, 173, 39, 118, 234,
                223, 129, 46, 220, 26, 67, 163, 20, 6, 203, 84, 191, 245, 146,
            ],
        }
    }
}
