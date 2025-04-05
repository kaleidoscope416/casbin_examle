# 认证后的用户身份应该加在哪
casbin_middleware 是一个中间件，读取 ServiceRequest 中 httprequest 中 httprequestinner 中 Extensions 中 CasbinVals。  
所以用户认证后应该把用户认证身份写成CasbinVals加到Extensions里  
ServiceRequest：封装了 httprequest 和 payload  
# 模型配置与策略配置
角色：用户 路径 http  
策略：用户 路径 http  
模型配置：给出角色，策略，判断，匹配方式的定义    
策略配置：存储每一个策略  
总结就是看你传进来的角色和策略匹不匹配    
解释g ，dom  
dom：域，似乎用不着，以option封装在 CasbinVals里
g： 相当于父类，比如各个用户id a,b,c可以通过g a admin 使得a可以进行admin的策略。
# 连接数据库
模型配置是一个简单的，存本地。策略可以存在数据库
# 策略设计
因为这个是比较后面的内容，等网关接口补齐才好设计
# 问题
如果权限不匹配，怎么返回关于错误的response

