use std::borrow::Cow;

use sqlx_core::database::Database;
use sqlx_core::decode::Decode;
use sqlx_core::encode::{Encode, IsNull};
use sqlx_core::error::BoxDynError;
use sqlx_core::types::Type;

use crate::{Utf8Path, Utf8PathBuf};

#[cfg(feature = "sqlx-sqlite")]
mod sqlite {
    use sqlx_sqlite::{Sqlite, SqliteArgumentValue, SqliteValueRef};

    use super::*;

    impl Type<Sqlite> for Utf8Path {
        fn type_info() -> <Sqlite as Database>::TypeInfo {
            <str as Type<Sqlite>>::type_info()
        }
    }

    impl<'q> Encode<'q, Sqlite> for &'q Utf8Path {
        fn encode_by_ref(
            &self,
            buf: &mut <Sqlite as Database>::ArgumentBuffer<'q>,
        ) -> Result<IsNull, BoxDynError> {
            <&str as Encode<Sqlite>>::encode_by_ref(&self.as_str(), buf)
        }
    }

    impl<'r> Decode<'r, Sqlite> for &'r Utf8Path {
        fn decode(value: SqliteValueRef<'r>) -> Result<Self, BoxDynError> {
            <&str as Decode<Sqlite>>::decode(value).map(Utf8Path::new)
        }
    }

    impl Type<Sqlite> for Utf8PathBuf {
        fn type_info() -> <Sqlite as Database>::TypeInfo {
            <String as Type<Sqlite>>::type_info()
        }
    }

    impl<'q> Encode<'q, Sqlite> for Utf8PathBuf {
        fn encode(self, buf: &mut Vec<SqliteArgumentValue<'q>>) -> Result<IsNull, BoxDynError>
        where
            Self: Sized,
        {
            buf.push(SqliteArgumentValue::Text(Cow::Owned(self.into_string())));

            Ok(IsNull::No)
        }

        fn encode_by_ref(
            &self,
            buf: &mut Vec<SqliteArgumentValue<'q>>,
        ) -> Result<IsNull, BoxDynError> {
            buf.push(SqliteArgumentValue::Text(Cow::Owned(
                self.as_str().to_owned(),
            )));

            Ok(IsNull::No)
        }
    }

    impl<'r> Decode<'r, Sqlite> for Utf8PathBuf {
        fn decode(value: SqliteValueRef<'r>) -> Result<Self, BoxDynError> {
            <String as Decode<Sqlite>>::decode(value).map(Utf8PathBuf::from)
        }
    }
}
