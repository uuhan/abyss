-- 好像没法在 loop 循环中进行 commit
-- 如果所有压缩或者解压缩都只能在一个事务里面执行, 很容易出错
create or replace function ticks_compress()
  returns table (
    decompressed varchar
  )
as $$

  declare chunk record;

begin

  for chunk in select * from show_chunks('ticks')
    loop

      begin
        raise notice '%s compressing', chunk.show_chunks;
        execute 'select compress_chunk($1, true);' using chunk.show_chunks;
        raise notice '%s compressed', chunk.show_chunks;
      end;

      decompressed := chunk.show_chunks;

      return next;

    end loop;
end;

$$ language plpgsql;

create or replace function ticks_decompress()
  returns table (
    decompressed varchar
  )
as $$

  declare chunk record;

begin

  for chunk in select * from show_chunks('ticks')
    loop

      begin
        raise notice '%s decompressing', chunk.show_chunks;
        execute 'select decompress_chunk($1, true);' using chunk.show_chunks;
        raise notice '%s decompressed', chunk.show_chunks;
      end;

      decompressed := chunk.show_chunks;

      return next;

    end loop;
end;

$$ language plpgsql;

