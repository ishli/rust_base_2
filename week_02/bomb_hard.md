Первичный шаг objdump -d bomb.hard > bomb.asm

2. делаем остановку в фазе 1

значение в адресе (из bomb.asm секция phase_1 видим mov    $0x402069, %esi)

(gdb) x /s 0x402069
0x402069:       "Rust is blazingly fast and memory-efficient"

итого Rust is blazingly fast and memory-efficient
фаза 1 пройдена

Фаза 2
сразу в секции фазы 2 видим адрес 0x4020a6 - исследуем его через gdb
(gdb) x/s 0x4020a6
0x4020a6:       "%d %d %d %d %d %d"

это шаблон для ввода 6 чисел (нам и предлагают ввести 6 числе)

далее 
cmpl   $0x1, -0x20(%rbp) - явно первое число это 1
далее Lphase2_loop - цикл
в нем
shl    $1, %ecx - умножение на 2 (через сдвиг)
т.е. числа 1 2 4 8 16 32
прошли фазу 2

Фаза 3
movabs $0x4020b2, %rsi
исследуем адрес 0x4020b2
(gdb) x/s 0x4020b2
0x4020b2:       "%d %d"
значит вводим два числа

видим 
sub    $0x5, %rax
ja     4013ad
в 4013ad - бомба срабатывает, значит наше число от 0 до 5, чтобы переход к бомбе не сработал

далее (сразу посомтрим ниже и видим 6 вызово вида cmpl   $0x1c4,-0x10(%rbp)- в них сранивается 2 число)
Из cmpl   $0x1c4, -0x10(%rbp) следует, что 452- кандидат на второе число, но не ясно про первое, кроме того,что оно >= 5
 
видим
  40130a:       48 89 45 e8             mov    %rax,-0x18(%rbp)
  40130e:       48 83 e8 05             sub    $0x5,%rax
  401312:       0f 87 95 00 00 00       ja     4013ad <phase_3+0xdd>
  401318:       48 8b 45 e8             mov    -0x18(%rbp),%rax
  40131c:       48 8b 04 c5 08 20 40    mov    0x402008(,%rax,8),%rax
  
  сохраняем значение rax в rbpm манипуляция с rax, потом его восстановление снова  и далее его изменение (0x402008 + %rax * 8)

делаем
break *phase_3++84
В rax 0x401396 
cmpl   $0x378,-0x10(%rbp)
значит 5 и 888 оба числа
прошли 

фаза 4
phase_4 
movabs $0x4020da,%rsi

исследуем 0x4020da
"%d %19s"

значит число и текст для ввода

видим еще mov    $0x4020f3,%esi
исследуем 0x4020f3
3:       "blazingpower"
значит фраза blazingpower
смотрим 
401483:       0f 8c 0a 00 00 00       jl     401493 <phase_4+0x53>
  401489:       83 7d f4 0c             cmpl   $0xc,-0xc(%rbp)
  40148d:       0f 8e 05 00 00 00       jle    401498 <phase_4+0x58>
  401493:       e8 e8 fc ff ff          call   401180 <explode_bomb>
  401498:       8b 7d f4                mov    -0xc(%rbp),%edi
  
  т.е. наше число >0 и <12
  
    40149b:       e8 30 ff ff ff          call   4013d0 <fib>
  4014a0:       83 f8 37                cmp    $0x37,%eax
  
  т.е. занчение от вызова fib (там фибоначи считают) = 55, значит наше число 10
  Итого ответ 10 и blazingpower
  прошли
  
Секретная фраза
исследуем movabs $0x4020b5,%rsi
  
(gdb) x/s 0x4020b5
0x4020b5:       "%d"
значит число

  посчитав все команды вида secret_phase+0x79
  видим
  cmpl   $0x539,-0x74(%rbp)
  это 1337 - попробуем
  
  
Enter the secret number: 1337
Secret phase defused! You have fully completed the game!
[Inferior 1 (process 2543436) exited normally]

прошли

ответы
Rust is blazingly fast and memory-efficient
1 2 4 8 16 32
5 888
10 blazingpower
1337