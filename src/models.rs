use crate::diesel::ExpressionMethods;
use borsh::{BorshDeserialize, BorshSerialize};
use diesel::{Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde::Serialize;
use solana_sdk::clock::UnixTimestamp;
use solana_sdk::pubkey::Pubkey;

use crate::schema::streams;
use crate::solana::get_accounts_and_update;

// to hold Data from stream PDA accounts
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
struct StreamData {
    pub start_time: UnixTimestamp,
    pub end_time: UnixTimestamp,
    pub receiver: Pubkey,
    pub lamports_withdrawn: u64,
    pub amount_second: u64,
    pub sender: Pubkey,
}

// Struct we want to store in the SQL db
#[derive(Debug, Queryable, Insertable, Serialize)]
#[table_name = "streams"]
pub struct Stream {
    pub pda_account: String,
    pub start_time: i64,
    pub end_time: i64,
    pub receiver: String,
    pub lamports_withdrawn: i64,
    pub amount_second: i64,
    pub sender: String,
    pub total_amount: i64,
}

impl Stream {
    pub fn new(pda_pubkey: String, pda_data: &Vec<u8>) -> Option<Self> {
        let stream_data = match StreamData::try_from_slice(pda_data) {
            Ok(a) => a,
            Err(e) => {
                println!(
                    "Failed to deserialize {} with error {:?}",
                    pda_pubkey.to_string(),
                    e
                );
                return None;
            }
        };

        Some(Stream {
            sender: stream_data.sender.to_string(),
            end_time: stream_data.end_time,
            receiver: stream_data.receiver.to_string(),
            lamports_withdrawn: stream_data.lamports_withdrawn as i64,
            start_time: stream_data.start_time,
            total_amount: (stream_data.end_time - stream_data.start_time)
                * stream_data.amount_second as i64,
            pda_account: pda_pubkey,
            amount_second: stream_data.amount_second as i64,
        })
    }

    // TODO get these get_all_with_x and only pass in our filter criteria.
    pub fn get_all_with_sender(pubkey: &String, conn: &PgConnection) -> Vec<Stream> {
        use crate::schema::streams::dsl::*;
        println!("get_all_with_sender called with pubkey {}", pubkey);
        //get_accounts_and_update();
        let stream_list = streams
            .filter(sender.eq(pubkey))
            .load::<Stream>(conn)
            .unwrap();
        if stream_list.len() > 0 {
            println!("streamlist first PDA: {}", stream_list[0].pda_account);
        } else {
            println!("streamlist is empty: []");
        }
        stream_list
    }
    pub fn get_all_with_receiver(pubkey: &String, conn: &PgConnection) -> Vec<Stream> {
        use crate::schema::streams::dsl::*;
        println!("get_all_with_receiver called with pubkey {}", pubkey);
        //get_accounts_and_update();
        let stream_list = streams
            .filter(receiver.eq(pubkey))
            .load::<Stream>(conn)
            .unwrap();
        if stream_list.len() > 0 {
            println!("streamlist first PDA: {}", stream_list[0].pda_account);
        } else {
            println!("streamlist is empty: []");
        }
        stream_list
    }
    pub fn get_all_with_pda(pubkey: &String, conn: &PgConnection) -> Vec<Stream> {
        use crate::schema::streams::dsl::*;
        println!("get_all_with_pda called with pubkey {}", pubkey);
        //get_accounts_and_update();
        let stream_list = streams
            .filter(pda_account.eq(pubkey))
            .load::<Stream>(conn)
            .unwrap();
        if stream_list.len() > 0 {
            println!("streamlist first PDA: {}", stream_list[0].pda_account);
        } else {
            println!("streamlist is empty: []");
        } 
        stream_list
    }
    fn id_is_present(id: &String, conn: &PgConnection) -> bool {
        use crate::schema::streams::dsl::*;
        match streams.find(id).first::<Stream>(conn) {
            Ok(_s) => true,
            _ => false,
        }
    }
    pub fn insert_or_update(stream: Stream, conn: &PgConnection) -> bool {
        if Stream::id_is_present(&stream.pda_account, conn) {
            use crate::schema::streams::dsl::{
                amount_second as a_s, end_time as e_t, lamports_withdrawn as l_w,
                pda_account as p_a, receiver as r, sender as s, streams, total_amount as t_a,
            };
            diesel::update(streams.find(stream.pda_account.clone()))
                .set((
                    a_s.eq(stream.amount_second),
                    e_t.eq(stream.end_time),
                    r.eq(stream.receiver),
                    p_a.eq(stream.pda_account),
                    s.eq(stream.sender),
                    l_w.eq(stream.lamports_withdrawn),
                    t_a.eq(stream.total_amount),
                    e_t.eq(stream.end_time),
                ))
                .execute(conn)
                .is_ok()
        } else {
            diesel::insert_into(crate::schema::streams::table)
                .values(&stream)
                .execute(conn)
                .is_ok()
        }
    }
}
