-- 查询 ip 前 10 名
SELECT ip, count(*) as total, cast(avg(len) as int) as avg_len FROM nginx GROUP BY ip ORDER BY total DESC LIMIT 10
-- 查询 UA 前 10 名
select ua, count(*) as total from nginx group by ua order by total desc limit 10
-- 查询访问最多的 url 前 10 名
select url, count(*) as total from nginx group by url order by total desc limit 10
-- 查询访问返回 body 长度前 10 名
select len, count(*) as total from nginx group by len order by total desc limit 10
-- 查询 HEAD 请求
select ip, date, url, code, ua from nginx where method = 'HEAD' limit 10
-- 查询状态码是 403 的请求
select ip, date, url, ua from nginx where code = 403 limit 10
-- 查询 UA 为空的请求
select ip, date, url, code from nginx where ua = '-' limit 10
-- 复杂查询，找返回 body 长度的 percentile 在 0.5-0.7 之间的数据
select * from (select ip, date, url, ua, len, PERCENT_RANK() OVER (ORDER BY len) as len_percentile from nginx where code = 200 order by len desc) as t where t.len_percentile > 0.5 and t.len_percentile < 0.7 order by t.len_percentile desc limit 10
