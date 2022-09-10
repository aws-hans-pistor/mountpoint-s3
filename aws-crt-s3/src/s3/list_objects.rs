//! The S3 ListObjectsV2 API

#![allow(unused)]

use crate::common::allocator::Allocator;
use crate::common::error::Error;
use crate::s3::client::Client;
use crate::s3::paginator::Paginator;
use crate::s3::s3_library_init;
use crate::{PtrExt, StringExt};
use aws_crt_s3_sys::*;
use std::ffi::OsStr;
use std::ptr::NonNull;

/// Parameters for a ListObjectsV2 request
#[derive(Debug)]
pub struct ListObjectsParams<'a> {
    client: &'a mut Client,
    bucket_name: &'a str,
    prefix: &'a str,
    delimiter: &'a str,
}

/// Initiate a new ListObjectsV2 request and return a paginator that can be used to receive the
/// results
pub fn initiate_list_objects(allocator: &mut Allocator, params: &ListObjectsParams) -> Result<Paginator, Error> {
    // Safety: aws_s3_initiate_list_objects makes copies of the strings we pass in here
    let inner = unsafe {
        let inner_params = aws_s3_list_objects_params {
            bucket_name: params.bucket_name.as_aws_byte_cursor(),
            prefix: params.prefix.as_aws_byte_cursor(),
            delimiter: params.delimiter.as_aws_byte_cursor(),
            ..Default::default()
        };

        aws_s3_initiate_list_objects(allocator.inner.as_ptr(), &inner_params).ok_or_last_error()?
    };

    Ok(Paginator { inner })
}