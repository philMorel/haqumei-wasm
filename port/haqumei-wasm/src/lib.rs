use haqumei::open_jtalk::Dictionary;
use haqumei::{AlignedDictionaryBytes, CandidateOptions, Haqumei as CoreHaqumei, HaqumeiOptions, IuPronunciation, NjdFeature, ProsodyFormat, UnicodeNormalization, update_global_dictionary};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

fn js_err<E: core::fmt::Display>(e: E) -> JsValue { JsValue::from_str(&e.to_string()) }
fn js<T: Serialize>(value: &T) -> Result<JsValue, JsValue> { serde_wasm_bindgen::to_value(value).map_err(js_err) }
fn texts(value: JsValue) -> Result<Vec<String>, JsValue> { serde_wasm_bindgen::from_value(value).map_err(js_err) }

#[derive(Default, Deserialize)]
#[serde(default)]
struct OptionsInput {
    normalize_unicode: Option<String>, use_read_as_pron: Option<bool>, revert_long_vowels: Option<bool>, revert_yotsugana: Option<bool>, normalize_iu: Option<String>,
    modify_filler_accent: Option<bool>, predict_nani: Option<bool>, predict_kana_english: Option<bool>, modify_context_reading: Option<bool>, modify_old_province_yomi: Option<bool>,
    restore_loanword_kana: Option<bool>, protect_user_dict_readings: Option<bool>, read_unknown_kanji: Option<bool>, modify_numeral_reading: Option<bool>, split_prefix_accent_phrase: Option<bool>,
    retreat_acc_nuc: Option<bool>, modify_acc_after_chaining: Option<bool>, process_odoriji: Option<bool>, use_allophones: Option<bool>, split_n_allophones: Option<bool>,
    split_n_before_palatal_affricate: Option<bool>, split_n_before_r: Option<bool>, split_q_allophones: Option<bool>, enable_final_glottal_stop: Option<bool>,
}

fn parse_options(value: Option<JsValue>) -> Result<HaqumeiOptions, JsValue> {
    let input = match value { Some(v) if !v.is_null() && !v.is_undefined() => serde_wasm_bindgen::from_value(v).map_err(js_err)?, _ => OptionsInput::default() };
    let mut out = HaqumeiOptions::default();
    if let Some(v)=input.normalize_unicode { out.normalize_unicode = match v.to_ascii_lowercase().as_str() { "none"|"none_" => UnicodeNormalization::None, "nfc" => UnicodeNormalization::Nfc, "nfkc" => UnicodeNormalization::Nfkc, _ => return Err(JsValue::from_str("normalize_unicode must be none, nfc, or nfkc")) }; }
    if let Some(v)=input.normalize_iu { out.normalize_iu = match v.to_ascii_lowercase().as_str() { "none"|"none_" => None, "iu" => Some(IuPronunciation::Iu), "yuu" => Some(IuPronunciation::Yuu), "kanji_iu"|"kanjiiu" => Some(IuPronunciation::KanjiIu), "kanji_yuu"|"kanjiyuu" => Some(IuPronunciation::KanjiYuu), "yuu_base"|"yuubase" => Some(IuPronunciation::YuuBase), "kanji_yuu_base"|"kanjiyuubase" => Some(IuPronunciation::KanjiYuuBase), _ => return Err(JsValue::from_str("invalid normalize_iu")) }; }
    macro_rules! set { ($($f:ident),* $(,)?) => { $(if let Some(v)=input.$f { out.$f=v; })* }; }
    set!(use_read_as_pron,revert_long_vowels,revert_yotsugana,modify_filler_accent,predict_nani,predict_kana_english,modify_context_reading,modify_old_province_yomi,restore_loanword_kana,protect_user_dict_readings,read_unknown_kanji,modify_numeral_reading,split_prefix_accent_phrase,retreat_acc_nuc,modify_acc_after_chaining,process_odoriji,use_allophones,split_n_allophones,split_n_before_palatal_affricate,split_n_before_r,split_q_allophones,enable_final_glottal_stop);
    Ok(out)
}

#[derive(Serialize)]
struct NjdOut<'a> { string:&'a str, pos:&'a str, pos_group1:&'a str, pos_group2:&'a str, pos_group3:&'a str, ctype:&'a str, cform:&'a str, orig:&'a str, read:&'a str, pron:&'a str, acc:i32, mora_size:i32, chain_rule:&'a str, chain_flag:i32 }
fn njd(v:&NjdFeature)->NjdOut<'_>{ NjdOut{string:&v.string,pos:&v.pos,pos_group1:&v.pos_group1,pos_group2:&v.pos_group2,pos_group3:&v.pos_group3,ctype:&v.ctype,cform:&v.cform,orig:&v.orig,read:&v.read,pron:&v.pron,acc:v.acc,mora_size:v.mora_size,chain_rule:&v.chain_rule,chain_flag:v.chain_flag} }
fn njds(v:&[NjdFeature])->Vec<NjdOut<'_>>{v.iter().map(njd).collect()}

#[derive(Default, Deserialize)] #[serde(default)]
struct CandidateInput { max_delta:Option<i64>, max_alternatives_per_branch:Option<usize>, max_candidates:Option<usize>, branch_on_unknown_words:Option<bool> }
fn candidate_options(value:Option<JsValue>)->Result<CandidateOptions,JsValue>{ let i=match value{Some(v) if !v.is_null()&&!v.is_undefined()=>serde_wasm_bindgen::from_value(v).map_err(js_err)?, _=>CandidateInput::default()}; let mut o=CandidateOptions::default(); if let Some(v)=i.max_delta{o.max_delta=v} if let Some(v)=i.max_alternatives_per_branch{o.max_alternatives_per_branch=v} if let Some(v)=i.max_candidates{o.max_candidates=v} if let Some(v)=i.branch_on_unknown_words{o.branch_on_unknown_words=v} Ok(o) }
fn prosody_format(v:Option<String>)->Result<ProsodyFormat,JsValue>{Ok(match v.as_deref().unwrap_or("default").to_ascii_lowercase().as_str(){"default"=>ProsodyFormat::Default,"prefix"=>ProsodyFormat::Prefix,"numeric"=>ProsodyFormat::Numeric,_=>return Err(JsValue::from_str("format must be default, prefix, or numeric"))})}

#[wasm_bindgen(js_name = Dictionary)]
pub struct JsDictionary { inner: Dictionary }

#[wasm_bindgen(js_class = Dictionary)]
impl JsDictionary {
    #[wasm_bindgen(js_name = from_bytes)]
    pub fn from_bytes(system:&[u8], chars:&[u8], matrix:&[u8])->Result<JsDictionary,JsValue>{
        Ok(Self{inner:Dictionary::from_binary_blobs(system,chars,matrix).map_err(js_err)?})
    }

    pub fn system_feature_count(&self)->Result<usize,JsValue>{
        self.inner.system_feature_count().map_err(js_err)
    }

    pub fn system_features(&self,start:usize,count:usize)->Result<JsValue,JsValue>{
        const MAX_BATCH:usize=512;
        if count>MAX_BATCH{return Err(JsValue::from_str("system feature batch exceeds 512 records"));}
        let total=self.inner.system_feature_count().map_err(js_err)?;
        let end=start.saturating_add(count).min(total);
        let mut out=Vec::with_capacity(end.saturating_sub(start));
        for index in start..end {
            if let Some(feature)=self.inner.system_feature_at(index).map_err(js_err)? { out.push(feature); }
        }
        js(&out)
    }

    pub fn system_surface_features(&self,surface:&str)->Result<JsValue,JsValue>{
        js(&self.inner.system_surface_features(surface).map_err(js_err)?)
    }

    pub fn system_feature_bytes(&self,start:usize,count:usize)->Result<Vec<u8>,JsValue>{
        const MAX_BATCH:usize=8192;
        if count>MAX_BATCH{return Err(JsValue::from_str("system feature byte batch exceeds 8192 records"));}
        let total=self.inner.system_feature_count().map_err(js_err)?;
        let end=start.saturating_add(count).min(total);
        let mut out=Vec::new();
        out.extend_from_slice(&(u32::try_from(end.saturating_sub(start)).map_err(js_err)?).to_le_bytes());
        for index in start..end {
            if let Some(feature)=self.inner.system_feature_at(index).map_err(js_err)? {
                let bytes=feature.as_bytes();
                out.extend_from_slice(&(u32::try_from(bytes.len()).map_err(js_err)?).to_le_bytes());
                out.extend_from_slice(bytes);
            }
        }
        Ok(out)
    }

    pub fn system_surface_feature_bytes(&self,surface:&str)->Result<Vec<u8>,JsValue>{
        let features=self.inner.system_surface_features(surface).map_err(js_err)?;
        let mut out=Vec::new();
        out.extend_from_slice(&(u32::try_from(features.len()).map_err(js_err)?).to_le_bytes());
        for feature in features {
            let bytes=feature.as_bytes();
            out.extend_from_slice(&(u32::try_from(bytes.len()).map_err(js_err)?).to_le_bytes());
            out.extend_from_slice(bytes);
        }
        Ok(out)
    }

    pub fn system_feature_bytes_matching(&self,start:usize,count:usize,pos:&str,pos1:&str)->Result<Vec<u8>,JsValue>{
        const MAX_SCAN:usize=65536;
        if count>MAX_SCAN{return Err(JsValue::from_str("system feature filtered scan exceeds 65536 records"));}
        let total=self.inner.system_feature_count().map_err(js_err)?;
        let end=start.saturating_add(count).min(total);
        let mut payload=Vec::new();
        let mut matched=0u32;
        for index in start..end {
            let Some(feature)=self.inner.system_feature_at(index).map_err(js_err)? else { continue; };
            let mut fields=feature.split(',');
            let mut field3=None;
            let mut field4=None;
            for field_index in 0..=4 {
                let Some(field)=fields.next() else { break; };
                if field_index==3 { field3=Some(field); }
                if field_index==4 { field4=Some(field); }
            }
            if !pos.is_empty() && field3!=Some(pos) { continue; }
            if !pos1.is_empty() && field4!=Some(pos1) { continue; }
            let bytes=feature.as_bytes();
            payload.extend_from_slice(&(u32::try_from(bytes.len()).map_err(js_err)?).to_le_bytes());
            payload.extend_from_slice(bytes);
            matched=matched.checked_add(1).ok_or_else(||JsValue::from_str("system feature match count overflow"))?;
        }
        let mut out=Vec::with_capacity(4+payload.len());
        out.extend_from_slice(&matched.to_le_bytes());
        out.extend_from_slice(&payload);
        Ok(out)
    }

    pub fn system_surface_reading_bytes_matching(&self,start:usize,count:usize,pos:&str,pos1:&str)->Result<Vec<u8>,JsValue>{
        const MAX_SCAN:usize=65536;
        if count>MAX_SCAN{return Err(JsValue::from_str("system surface-reading filtered scan exceeds 65536 records"));}
        let total=self.inner.system_feature_count().map_err(js_err)?;
        let end=start.saturating_add(count).min(total);
        let mut payload=Vec::new();
        let mut matched=0u32;
        for index in start..end {
            let Some(feature)=self.inner.system_feature_at(index).map_err(js_err)? else { continue; };
            let mut fields=feature.split(',');
            let mut field3=None;
            let mut field4=None;
            let mut surface=None;
            let mut reading=None;
            for field_index in 0..=10 {
                let Some(field)=fields.next() else { break; };
                match field_index {
                    3 => field3=Some(field),
                    4 => field4=Some(field),
                    9 => surface=Some(field),
                    10 => reading=Some(field),
                    _ => {}
                }
            }
            if !pos.is_empty() && field3!=Some(pos) { continue; }
            if !pos1.is_empty() && field4!=Some(pos1) { continue; }
            let (Some(surface),Some(reading))=(surface,reading) else { continue; };
            let surface_bytes=surface.as_bytes();
            let reading_bytes=reading.as_bytes();
            payload.extend_from_slice(&(u32::try_from(surface_bytes.len()).map_err(js_err)?).to_le_bytes());
            payload.extend_from_slice(surface_bytes);
            payload.extend_from_slice(&(u32::try_from(reading_bytes.len()).map_err(js_err)?).to_le_bytes());
            payload.extend_from_slice(reading_bytes);
            matched=matched.checked_add(1).ok_or_else(||JsValue::from_str("system surface-reading match count overflow"))?;
        }
        let mut out=Vec::with_capacity(4+payload.len());
        out.extend_from_slice(&matched.to_le_bytes());
        out.extend_from_slice(&payload);
        Ok(out)
    }

    pub fn system_has_surface(&self,surface:&str)->Result<bool,JsValue>{
        Ok(!self.inner.system_surface_features(surface).map_err(js_err)?.is_empty())
    }

    pub fn system_surface_id(&self,surface:&str)->Result<Option<u32>,JsValue>{
        self.inner.system_surface_id(surface).map_err(js_err)
    }
}

#[wasm_bindgen(js_name = DictionaryBlobLoader)]
pub struct DictionaryBlobLoader {
    system_len:usize, char_len:usize, matrix_len:usize, received:usize,
    system:AlignedDictionaryBytes, chars:Vec<u8>, matrix:Vec<u8>
}

#[wasm_bindgen(js_class = DictionaryBlobLoader)]
impl DictionaryBlobLoader {
    #[wasm_bindgen(constructor)]
    pub fn new(system_len:usize,char_len:usize,matrix_len:usize)->Self{
        Self{system_len,char_len,matrix_len,received:0,system:AlignedDictionaryBytes::with_capacity(system_len),chars:Vec::with_capacity(char_len),matrix:Vec::with_capacity(matrix_len)}
    }
    pub fn append(&mut self, chunk:&[u8])->Result<(),JsValue>{
        if self.received+chunk.len()>self.expected_bytes(){return Err(JsValue::from_str("dictionary blob is larger than declared sizes"));}
        let mut at=0usize;
        while at<chunk.len(){
            let global=self.received;
            if global<self.system_len {
                let n=(self.system_len-global).min(chunk.len()-at);
                self.system.extend_from_slice(&chunk[at..at+n]); at+=n; self.received+=n;
            } else if global<self.system_len+self.char_len {
                let local=global-self.system_len;
                let n=(self.char_len-local).min(chunk.len()-at);
                self.chars.extend_from_slice(&chunk[at..at+n]); at+=n; self.received+=n;
            } else {
                let local=global-self.system_len-self.char_len;
                let n=(self.matrix_len-local).min(chunk.len()-at);
                self.matrix.extend_from_slice(&chunk[at..at+n]); at+=n; self.received+=n;
            }
        }
        Ok(())
    }
    pub fn received_bytes(&self)->usize{self.received}
    pub fn expected_bytes(&self)->usize{self.system_len+self.char_len+self.matrix_len}
    pub fn finish(&mut self)->Result<JsDictionary,JsValue>{
        if self.received!=self.expected_bytes(){return Err(JsValue::from_str("dictionary blob is incomplete"));}
        let system=core::mem::replace(&mut self.system, AlignedDictionaryBytes::with_capacity(0));
        let chars=core::mem::take(&mut self.chars);
        let matrix=core::mem::take(&mut self.matrix);
        Ok(JsDictionary{inner:Dictionary::from_aligned_binary_blobs(system,&chars,&matrix).map_err(js_err)?})
    }
}

#[wasm_bindgen(js_name = update_global_dictionary)]
pub fn js_update_global_dictionary(dict:&JsDictionary){ update_global_dictionary(dict.inner.clone()); }

#[wasm_bindgen(js_name = Haqumei)]
pub struct JsHaqumei { inner: CoreHaqumei }

#[wasm_bindgen(js_class = Haqumei)]
impl JsHaqumei {
    #[wasm_bindgen(constructor)]
    pub fn new(options:Option<JsValue>)->Result<JsHaqumei,JsValue>{
        Ok(Self{inner:CoreHaqumei::with_options(parse_options(options)?).map_err(js_err)?})
    }
    #[wasm_bindgen(js_name = from_dictionary)]
    pub fn from_dictionary(dict:&JsDictionary,options:Option<JsValue>)->Result<JsHaqumei,JsValue>{
        Ok(Self{inner:CoreHaqumei::from_dictionary(dict.inner.clone(),parse_options(options)?).map_err(js_err)?})
    }
    pub fn run_frontend(&mut self,text:&str)->Result<JsValue,JsValue>{
        let v=self.inner.run_frontend(text).map_err(js_err)?; js(&njds(&v))
    }
    pub fn run_frontend_batch(&mut self,input:JsValue)->Result<JsValue,JsValue>{
        let t=texts(input)?; let v=self.inner.run_frontend_batch(&t).map_err(js_err)?;
        let out:Vec<_>=v.iter().map(|x|njds(x)).collect(); js(&out)
    }
    pub fn run_frontend_detailed(&mut self,text:&str)->Result<JsValue,JsValue>{
        let(v,m)=self.inner.run_frontend_detailed(text).map_err(js_err)?; js(&(njds(&v),m))
    }
    pub fn run_frontend_detailed_batch(&mut self,input:JsValue)->Result<JsValue,JsValue>{
        let t=texts(input)?; let v=self.inner.run_frontend_detailed_batch(&t).map_err(js_err)?;
        let out:Vec<_>=v.iter().map(|(n,m)|(njds(n),m)).collect(); js(&out)
    }
    pub fn extract_fullcontext(&mut self,text:&str)->Result<JsValue,JsValue>{
        js(&self.inner.extract_fullcontext(text).map_err(js_err)?)
    }
    pub fn extract_fullcontext_batch(&mut self,input:JsValue)->Result<JsValue,JsValue>{
        let t=texts(input)?; js(&self.inner.extract_fullcontext_batch(&t).map_err(js_err)?)
    }
    pub fn extract_fullcontext_string(&mut self,text:&str)->Result<JsValue,JsValue>{
        js(&self.inner.extract_fullcontext_string(text).map_err(js_err)?)
    }
    pub fn extract_fullcontext_string_batch(&mut self,input:JsValue)->Result<JsValue,JsValue>{
        let t=texts(input)?; js(&self.inner.extract_fullcontext_string_batch(&t).map_err(js_err)?)
    }
    pub fn g2p(&mut self,text:&str)->Result<JsValue,JsValue>{
        let v=self.inner.g2p(text).map_err(js_err)?;
        js(&v.iter().map(|p|p.as_str()).collect::<Vec<_>>())
    }
    pub fn g2p_batch(&mut self,input:JsValue)->Result<JsValue,JsValue>{
        let t=texts(input)?; let v=self.inner.g2p_batch(&t).map_err(js_err)?;
        let o:Vec<Vec<&str>>=v.iter().map(|x|x.iter().map(|p|p.as_str()).collect()).collect(); js(&o)
    }
    pub fn g2p_detailed(&mut self,text:&str)->Result<JsValue,JsValue>{
        let v=self.inner.g2p_detailed(text).map_err(js_err)?;
        js(&v.iter().map(|p|p.as_str()).collect::<Vec<_>>())
    }
    pub fn g2p_detailed_batch(&mut self,input:JsValue)->Result<JsValue,JsValue>{
        let t=texts(input)?; let v=self.inner.g2p_detailed_batch(&t).map_err(js_err)?;
        let o:Vec<Vec<&str>>=v.iter().map(|x|x.iter().map(|p|p.as_str()).collect()).collect(); js(&o)
    }
    pub fn g2k(&mut self,text:&str)->Result<String,JsValue>{self.inner.g2k(text).map_err(js_err)}
    pub fn g2k_batch(&mut self,input:JsValue)->Result<JsValue,JsValue>{
        let t=texts(input)?; js(&self.inner.g2k_batch(&t).map_err(js_err)?)
    }
    pub fn g2k_per_word(&mut self,text:&str)->Result<JsValue,JsValue>{
        js(&self.inner.g2k_per_word(text).map_err(js_err)?)
    }
    pub fn g2k_per_word_batch(&mut self,input:JsValue)->Result<JsValue,JsValue>{
        let t=texts(input)?; js(&self.inner.g2k_per_word_batch(&t).map_err(js_err)?)
    }
    pub fn g2p_prosody(&mut self,text:&str,format:Option<String>)->Result<JsValue,JsValue>{
        js(&self.inner.g2p_prosody_with_options(text,prosody_format(format)?).map_err(js_err)?)
    }
    pub fn g2p_prosody_batch(&mut self,input:JsValue,format:Option<String>)->Result<JsValue,JsValue>{
        let t=texts(input)?;
        js(&self.inner.g2p_prosody_with_options_batch(&t,prosody_format(format)?).map_err(js_err)?)
    }
    pub fn g2p_per_word(&mut self,text:&str)->Result<JsValue,JsValue>{
        let v=self.inner.g2p_per_word(text).map_err(js_err)?;
        let o:Vec<Vec<&str>>=v.iter().map(|x|x.iter().map(|p|p.as_str()).collect()).collect(); js(&o)
    }
    pub fn g2p_per_word_batch(&mut self,input:JsValue)->Result<JsValue,JsValue>{
        let t=texts(input)?; let v=self.inner.g2p_per_word_batch(&t).map_err(js_err)?;
        let o:Vec<Vec<Vec<&str>>>=v.iter().map(|a|a.iter().map(|b|b.iter().map(|p|p.as_str()).collect()).collect()).collect(); js(&o)
    }
    pub fn g2p_mapping(&mut self,text:&str)->Result<JsValue,JsValue>{
        js(&self.inner.g2p_mapping(text).map_err(js_err)?)
    }
    pub fn g2p_mapping_batch(&mut self,input:JsValue)->Result<JsValue,JsValue>{
        let t=texts(input)?; js(&self.inner.g2p_mapping_batch(&t).map_err(js_err)?)
    }
    pub fn g2p_mapping_detailed(&mut self,text:&str)->Result<JsValue,JsValue>{
        js(&self.inner.g2p_mapping_detailed(text).map_err(js_err)?)
    }
    pub fn g2p_mapping_detailed_batch(&mut self,input:JsValue)->Result<JsValue,JsValue>{
        let t=texts(input)?; js(&self.inner.g2p_mapping_detailed_batch(&t).map_err(js_err)?)
    }
    pub fn g2p_mapping_prosody(&mut self,text:&str)->Result<JsValue,JsValue>{
        js(&self.inner.g2p_mapping_prosody(text).map_err(js_err)?)
    }
    pub fn g2p_mapping_prosody_batch(&mut self,input:JsValue)->Result<JsValue,JsValue>{
        let t=texts(input)?; js(&self.inner.g2p_mapping_prosody_batch(&t).map_err(js_err)?)
    }
    pub fn g2p_candidates(&mut self,text:&str,options:Option<JsValue>)->Result<JsValue,JsValue>{
        js(&self.inner.g2p_candidates_with_options(text,candidate_options(options)?).map_err(js_err)?)
    }
    pub fn g2p_candidates_detailed(&mut self,text:&str,options:Option<JsValue>)->Result<JsValue,JsValue>{
        js(&self.inner.g2p_candidates_detailed_with_options(text,candidate_options(options)?).map_err(js_err)?)
    }
    pub fn g2p_candidates_prosody(&mut self,text:&str,options:Option<JsValue>)->Result<JsValue,JsValue>{
        js(&self.inner.g2p_candidates_prosody_with_options(text,candidate_options(options)?).map_err(js_err)?)
    }
    pub fn g2p_candidates_batch(&mut self,input:JsValue,options:Option<JsValue>)->Result<JsValue,JsValue>{
        let t=texts(input)?;
        js(&self.inner.g2p_candidates_with_options_batch(&t,candidate_options(options)?).map_err(js_err)?)
    }
}

#[derive(Serialize)]
struct CapabilityInfo {
    version: &'static str,
    predict_nani: bool,
    predict_kana_english: bool,
    dictionary_in_memory: bool,
}

#[wasm_bindgen]
pub fn capabilities()->Result<JsValue,JsValue>{
    js(&CapabilityInfo {
        version: "0.12.0",
        predict_nani: true,
        predict_kana_english: false,
        dictionary_in_memory: true,
    })
}
