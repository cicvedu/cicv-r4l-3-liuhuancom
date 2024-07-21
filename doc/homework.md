### 作业

#### 作业1 

![image](https://github.com/user-attachments/assets/bbcada2f-1573-430a-b2e3-6e3a23e5082b)


#### 作业2
![image](https://github.com/user-attachments/assets/c518fb2e-e62d-40eb-a1bb-d9a848f3ca03)

![image](https://github.com/user-attachments/assets/d2685ec0-4c6b-4363-9130-3975e85b01f5)

![image](https://github.com/user-attachments/assets/9f326788-d454-463b-85cf-9ce6aa9e8b9e)


#### 作业3

![image](https://github.com/user-attachments/assets/21bcb3a5-13b9-450c-89fa-18b920c4cb56)
![image](https://github.com/user-attachments/assets/d3b55fc2-8e99-465b-9938-31801cbe9354)


#### 作业4


#### 作业5

![image](https://github.com/user-attachments/assets/992a3798-8aad-46b5-a249-21a710847ad5)

Q：作业5中的字符设备/dev/cicv是怎么创建的？它的设备号是多少？它是如何与我们写的字符设备驱动关联上的？ 

build_image.sh脚本中有条命令设置的 命令是 `echo "mknod /dev/cicv c 248 0" >> etc/init.d/rcS`  
由mknod的命令可以知道 主设备是248  
是通过主设备号关联的  

